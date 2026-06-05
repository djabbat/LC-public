# MCAOA Simulator — Minimal Reproducible Example

**Instrucciones para clonar, compilar y ejecutar el simulador MCAOA con el módulo EDC**

---

## 1. Requisitos

- **Rust toolchain** (rustc ≥ 1.70, cargo ≥ 1.70). Instalar: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Git**
- ~1.5 GB de espacio en disco (dependencias de compilación)
- Sistema operativo: Linux, macOS o Windows (WSL)

## 2. Clonar el repositorio

```bash
git clone -b mcaoa-v3.2 https://github.com/djabbat/LC.git
cd LC/MCAOA
```

## 3. Compilar

```bash
cargo build --release
```

Esto compilará todas las crates del workspace (mcoa_core, mcoa_simulation, mcoa_cli, etc.). La primera compilación puede tardar 3–10 minutos. Compilaciones posteriores serán más rápidas.

## 4. Ejecutar el test EDC

El test `edc_thyroid_accelerates_mito_and_proteostasis` demuestra el efecto del módulo EDC sobre los contadores:

```bash
cargo test --package mcoa_simulation edc_thyroid -- --nocapture
```

**Salida esperada:** el test pasa sin errores. Verifica que:

- El daño mitocondrial con EDC es **mayor** que sin EDC
- El daño de proteostasis con EDC es **mayor** que sin EDC
- La carga tisular (tissue_load) con EDC es **mayor** que sin EDC

## 5. Ejecutar una simulación completa con EDC

```bash
# Simular 500 divisiones de células HSC con exposición EDC tiroidea 0.6
cargo run --release --package mcoa_cli -- \
    --tissue hsc \
    --steps 500 \
    --days-per-step 7 \
    --edc-exposure 0.6 \
    --edc-target thyroid \
    --output resultados_edc.csv

# Ver los resultados
cat resultados_edc.csv | column -t -s, | head -10
```

**Parámetros disponibles:**
| Parámetro | Valores | Descripción |
|-----------|---------|-------------|
| `--tissue` | `fibroblast`, `hsc`, `neuron`, `hepatocyte`, `beta_cell`, `cd8_t_memory` | Tipo de tejido |
| `--steps` | entero positivo | Número de pasos de simulación |
| `--days-per-step` | flotante positivo | Duración de cada paso en días |
| `--edc-exposure` | 0.0 – 1.0 | Nivel de exposición normalizada |
| `--edc-target` | `none`, `thyroid`, `general` | Tipo de EDC |
| `--output` | ruta de archivo | Archivo CSV de salida |

## 6. Generar comparación EDC vs. basal

```bash
# Sin EDC
cargo run --release --package mcoa_cli -- \
    --tissue hsc --steps 500 --days-per-step 7 \
    --edc-exposure 0.0 --output basal.csv

# Con EDC
cargo run --release --package mcoa_cli -- \
    --tissue hsc --steps 500 --days-per-step 7 \
    --edc-exposure 0.8 --edc-target thyroid --output edc_alta.csv

# Comparar tissue_load entre basal y con EDC
echo "=== Último paso basal ===" && tail -1 basal.csv
echo "=== Último paso EDC ===" && tail -1 edc_alta.csv
```

## 7. Reproducir la figura del preprint (tissue_load)

El preprint MCOA (DOI: 10.5281/zenodo.20055806) muestra la evolución de L_tissue para 6 tipos tisulares. Para reproducirla:

```bash
# Ejecutar para todos los tejidos con configuración estándar
for tissue in fibroblast hsc neuron hepatocyte beta_cell cd8_t_memory; do
    cargo run --release --package mcoa_cli -- \
        --tissue $tissue --steps 1000 --days-per-step 3.65 \
        --output "${tissue}_sim.csv"
done

# Los archivos CSV contienen t_years (tiempo en años) y tissue_load
```

## 8. Código fuente del módulo EDC

Archivo principal: `crates/mcoa_simulation/src/lib.rs`

Funciones clave:
- `edc_modulate(counter, exposure, target) → f64` — calcula el multiplicador de drift
- `step(states, tissue, ..., edc_exposure, edc_target)` — avanza la simulación un paso
- `run(tissue, n_steps, ..., edc_exposure, edc_target) → Vec<SimulationRecord>` — simulación completa

## 9. Licencia

**MIT License** — el código es completamente abierto y libre para cualquier uso, modificación y redistribución. Solo se solicita la cita académica:

> Tqemaladze J. (2026) The Multi-Counter Architecture of Organismal Aging. eLife-RP-RA-2026-111885. DOI: 10.5281/zenodo.20055806

## 10. Soporte

Para cualquier incidencia o duda:
- GitHub Issues: https://github.com/djabbat/LC/issues
- Email: jaba@longevity.ge
- WhatsApp: +995 555 185 161
