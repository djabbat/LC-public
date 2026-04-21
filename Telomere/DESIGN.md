# Code Architecture & API for the Telomere Counter

## 1. Обзор архитектуры

Теломерный счётчик реализован как независимый модуль (класс `TelomereCounter`) в рамках симуляционной платформы MCOA. Его основная задача:
1.  **Обновление состояния:** Рассчитывать текущее значение дефицита `D₂` на основе истории делений `n`, хронологического времени `t` и, в будущем, состояния связанных счётчиков.
2.  **Расчёт вклада:** Преобразовывать `D₂` в вклад в общую нагрузку старения ткани `L_tissue` согласно функции `f₂` и весу `w₂`.
3.  **Предоставление интерфейсов:** Для связи с симулятором клеточного цикла (обновление `n`), симулятором окислительного стресса (влияние на `β₂`) и другими модулями.

## 2. Файловая структура

```
mcga_framework/               # Корень проекта MCOA
├── counters/
│   ├── __init__.py
│   ├── base_counter.py      # Абстрактный класс BaseCounter
│   └── telomere/
│       ├── __init__.py
│       ├── counter.py       # Основной класс TelomereCounter
│       ├── kinetics.py      # Функции для расчёта dD2/dt, f₂(D₂)
│       ├── parameters.py    # Константы (α₂, β₂, τ₂, n₂*) и default weights
│       └── tests/
│           └── test_telomere.py
├── tissue_models/           # Модели тканей, определяющие веса w_i(tissue)
├── simulator.py             # Главный симулятор, координирующий все счётчики
└── utils/
    └── loggers.py
```

## 3. API Contracts (Интерфейсы)

### 3.1. Класс `TelomereCounter` (counters/telomere/counter.py)

```python
class TelomereCounter(BaseCounter):
    """
    Implements Counter #2: Telomere Shortening.
    State variable: D₂ (telomere length deficit in bp).
    """

    def __init__(self,
                 initial_deficit: float = -12000.0,  # D₂,₀
                 alpha: float = 100.0,               # α₂ [bp/PD]
                 beta: float = 40.0,                 # β₂ [bp]
                 tau: float = 0.166,                 # τ₂ [years] (~2 months)
                 n_star: float = 50.0,               # n₂* [PD]
                 d_critical: float = 5000.0):        # D₂,critical [bp] for f₂
        """
        Initialize the telomere counter with canonical or custom parameters.
        """
        super().__init__(counter_id=2, name="TelomereShortening")
        self.alpha = alpha
        self.beta = beta
        self.tau = tau
        self.n_star = n_star
        self.d_critical = d_critical
        self.state = {
            'D': initial_deficit,  # Current deficit [bp]
            'n': 0.0,              # Cumulative population doublings [PD]
            't': 0.0               # Chronological time [years]
        }

    def update(self, dt: float, dn: float = 0.0, coupling_inputs: Dict[str, float] = None) -> float:
        """
        Update the internal state over a time step dt.
        Args:
            dt: Elapsed chronological time [years].
            dn: Change in population doublings during dt [PD].
            coupling_inputs: Dictionary with keys like 'ROS_level', 'proteostasis_deficit'
                             providing values from other counters to influence beta_effective.
        Returns:
            The new value of the state deficit D₂.
        """
        # 1. Calculate effective beta if there are coupling inputs
        beta_eff = self.beta
        if coupling_inputs:
            # Example: Linear coupling to ROS (Counter #3)
            ros_level = coupling_inputs.get('ROS_level', 0.0)
            # Assume coupling coefficient is embedded in scaling. This is a placeholder.
            # A real implementation would use a defined coupling function.
            # beta_eff = self.beta * (1.0 + self.gamma_23 * ros_level)
            pass

        # 2. Update state using the master equation in differential form (Euler step)
        # dD_dt = (self.alpha / self.n_star) * (dn/dt) + (beta_eff / self.tau)
        # Since dn is provided for the step, we add the division-dependent loss directly.
        division_loss = self.alpha * (dn / self.n_star)
        time_loss = (beta_eff / self.tau) * dt

        self.state['D'] += division_loss + time_loss
        self.state['n'] += dn
        self.state['t'] += dt

        return self.state['D']

    def get_load_contribution(self, tissue_type: str) -> float:
        """
        Calculate this counter's contribution to the aging load of a specific tissue.
        Args:
            tissue_type: String identifier (e.g., 'blood', 'skin', 'neuron').
        Returns:
            Load contribution L₂ = w₂(tissue) * f₂(D₂).
        """
        from ..parameters import TISSUE_WEIGHTS  # Would be defined elsewhere
        w = TISSUE_WEIGHTS.get(tissue_type, {}).get(self.counter_id, 0.0)
        f = self._scaling_function(self.state['D'])
        return w * f

    def _scaling_function(self, D: float) -> float:
        """
        Internal scaling function f₂(D).
        Simple linear ramp from 0 to 1 as deficit approaches critical.
        """
        if D >= 0:
            return 1.0  # Deficit is positive (loss beyond critical)
        elif D <= -self.d_critical:
            return 0.0  # No significant deficit
        else:
            # Linear increase from 0 to 1 as D goes from -d_critical to 0
            return -D / self.d_critical

    def get_state(self) -> Dict[str, float]:
        """Return a copy of the current state dictionary."""
        return self.state.copy()
```

### 3.2. Модуль параметров (counters/telomere/parameters.py)

```python
"""
Canonical parameters for the Telomere Counter.
All values are from PARAMETERS.md.
"""

# Core kinetic parameters (ranges from literature)
ALPHA_RANGE = (50.0, 200.0)          # α₂ [bp/PD]
BETA_RANGE = (20.0, 50.0)            # β₂ [bp]
TAU_RANGE = (0.083, 0.25)            # τ₂ [years] (1-3 months)
N_STAR_RANGE = (40.0, 60.0)          # n₂* [PD]
INITIAL_DEFICIT_RANGE = (-15000.0, -10000.0)  # D₂,₀ [bp]

# Default values (midpoints of ranges)
ALPHA_DEFAULT