# CONCEPT — Automated Microscopy (BioSense)

## Назначение
Автоматизированный пайплайн для микроскопического анализа изображений центриолей.

## Контекст
Часть BioSense — федеративной системы клинического обучения. Микроскопия производит изображения делящихся клеток; automated-microscopy обрабатывает их.

## Компоненты
1. **Захват изображений** — управление микроскопом (OpenSPIM, NanoJ-Fluidics)
2. **Обработка** — сегментация центриолей, трекинг mitotic spindle
3. **Анализ** — подсчёт центриолей, определение «возраста» по GFP-метке
4. **Экспорт** — данные в CDATA/CellLineageTree для lineage reconstruction

## Технологии
- Python/OpenCV для обработки изображений
- Связь с BioSense API (Rust backend)
- Входные данные: .tiff, .czi (Zeiss), .nd2 (Nikon)

## Связи
- **Родитель:** LC/BioSense/instruments
- **Потребитель:** CDATA/CellLineageTree, PhD/microscope
- **Стандарт:** OpenSPIM (Pitrone et al., 2013)

## Статус
🟡 Концепт. Оборудование не приобретено. Требуется доступ к микроскопу.
