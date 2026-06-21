#!/usr/bin/env python3
"""Визуализация результатов Organismal Aging симуляции."""

import csv
import sys
import matplotlib.pyplot as plt
import numpy as np

def plot_simulation(csv_path: str = None):
    """Построить графики старения из CSV-файла."""
    
    if csv_path:
        with open(csv_path) as f:
            reader = csv.DictReader(f)
            rows = list(reader)
    else:
        # Читаем из stdin
        reader = csv.DictReader(sys.stdin)
        rows = list(reader)
    
    age = [float(r['age']) for r in rows]
    s_centriole = [float(r['S_centriole']) for r in rows]
    fi = [float(r['fi']) for r in rows]
    
    tissues = ['epidermis_L', 'gut_L', 'liver_L', 'neurons_L',
               'hsc_L', 'heart_L', 'endothelium_L', 'bone_L']
    tissue_names = ['Эпидермис', 'Кишечник', 'Печень', 'Нейроны',
                    'HSC', 'Сердце', 'Эндотелий', 'Кость']
    
    tissue_data = {name: [float(r[key]) for r in rows] 
                   for key, name in zip(tissues, tissue_names)}
    
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    
    # 1. Энтропия центриоли
    ax = axes[0, 0]
    ax.plot(age, s_centriole, 'r-', linewidth=2, label='S_centriole(t)')
    ax.fill_between(age, 0, s_centriole, alpha=0.2, color='red')
    ax.set_xlabel('Возраст (лет)')
    ax.set_ylabel('Энтропия центриоли')
    ax.set_title('Уровень #1: Центриоль — накопитель энтропии')
    ax.set_ylim(0, 1.05)
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    # 2. Бремя тканей
    ax = axes[0, 1]
    colors = plt.cm.tab10(np.linspace(0, 1, len(tissue_names)))
    for (name, data), color in zip(tissue_data.items(), colors):
        ax.plot(age, data, linewidth=1.5, label=name, color=color)
    ax.axhline(y=0.60, color='red', linestyle='--', alpha=0.5, label='L_crit = 0.60')
    ax.axhline(y=0.55, color='orange', linestyle='--', alpha=0.5, label='L_crit нейронов = 0.55')
    ax.set_xlabel('Возраст (лет)')
    ax.set_ylabel('Бремя ткани L(t)')
    ax.set_title('Уровень #3: Старение 8 тканей')
    ax.set_ylim(0, 1.05)
    ax.grid(True, alpha=0.3)
    ax.legend(fontsize=8, loc='lower right')
    
    # 3. Frailty Index
    ax = axes[1, 0]
    ax.plot(age, fi, 'b-', linewidth=2, label='FI(t) = 0.7 × L_max')
    ax.fill_between(age, 0, fi, alpha=0.15, color='blue')
    ax.axhline(y=0.21, color='green', linestyle='--', alpha=0.5, label='FI=0.21 (healthy)')
    ax.axhline(y=0.42, color='red', linestyle='--', alpha=0.5, label='FI=0.42 (disease)')
    ax.set_xlabel('Возраст (лет)')
    ax.set_ylabel('Frailty Index')
    ax.set_title('Frailty Index (Rockwood 2005)')
    ax.set_ylim(0, 0.8)
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    # 4. Повреждения счётчиков
    ax = axes[1, 1]
    counter_keys = ['telomere_D', 'mito_D', 'epigen_D', 'proteo_D']
    counter_names = ['#2 Теломеры', '#3 Митохондрии', '#4 Эпигенетика', '#5 Протеостаз']
    counter_colors = ['blue', 'red', 'green', 'purple']
    
    for key, name, color in zip(counter_keys, counter_names, counter_colors):
        data = [float(r[key]) for r in rows]
        ax.plot(age, data, linewidth=1.5, label=name, color=color)
    
    ax.set_xlabel('Возраст (лет)')
    ax.set_ylabel('Повреждение D(t)')
    ax.set_title('Уровень #2: 5 счётчиков MCAOA')
    ax.grid(True, alpha=0.3)
    ax.legend()
    
    plt.suptitle('Organismal Aging — Симуляция жизни человека (120 лет)', 
                 fontsize=14, fontweight='bold', y=0.98)
    plt.tight_layout()
    plt.savefig('organismal_aging_simulation.png', dpi=150, bbox_inches='tight')
    print("График сохранён: organismal_aging_simulation.png")
    plt.show()

if __name__ == '__main__':
    csv_path = sys.argv[1] if len(sys.argv) > 1 else None
    plot_simulation(csv_path)
