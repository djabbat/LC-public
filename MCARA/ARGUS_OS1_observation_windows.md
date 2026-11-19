# ARGUS-OS1 — Окна наблюдения и late eliminators (закрытие Л4 и failure mode «окно vs финальная судьба»)

**Дата:** 2026-11-11
**Автор дизайна:** Jaba Tqemaladze (GLA) + pi
**Источник-триггер:** Yao 2026 §7.2/§4.1 — «The primary imaging window ends at roughly the 100-cell stage, before the comma stage at which much of the elimination in C. elegans occurs (Kalbfuss & Gönczy, 2023a). Cells scored as retained within the window may eliminate later. The design therefore reports a snapshot, flags late eliminators, and extends imaging in a subset; the claim is about the window, not about final fate, and is stated as such.»
**Мастер:** `~/Desktop/Services/docs/ANALYSIS_2026-11-11_ARGUS_Systems_Analysis_Yao_Ming.md` · чек-лист: `LC/MCARA/ARGUS_YAO2026_limitations_checklist.md` (Л4)
**Смежные:** `ARGUS_Protocol_Hayflick_Limit_Immortalized_Fibroblasts.md` (§1.1a — горизонт/стоп-критерий для immortalized) · `MCARA/docs/Celegans_Iron_Stain_Protocol.md` (Heidenhain-верификация эндпоинтов)
**Статус:** 🟡 дизайн-документ

---

## 1. Проблема
Первичное imaging-окно OS1 заканчивается примерно на **100-клеточной стадии**, а значительная часть элиминации центриолей в C. elegans происходит **позже (к комма-стадии)** (Kalbfuss & Gönczy 2023a). Поэтому:
- клетки, классифицированные как «retained» внутри окна, могут элиминировать ПОЗЖЕ окна;
- вывод OS1 — про **окно**, а не про финальную судьбу.

## 2. Дизайн-меры (обязательные в OS1-протоколе)
| № | Мера | Реализация |
|---|---|---|
| M1 | **Snapshot-report** | Первичный отчёт формулирует claim «в окне (до ~100-cell)», НЕ «финальная судьба». Каждая таблица/фигура подписана границей окна. |
| M2 | **Flag late eliminators** | Клетки, у которых элиминация регистрируется сразу за границей окна (позднее элиминирующие), помечаются отдельным статусом; не смешивать с «retained в окне». |
| M3 | **Продление imaging в подмножестве** | В пред-зарегистрированной подвыборке эмбрионов (напр. 20–30 из N=100) продлить imaging до комма-стадии и далее; сравнить долю late eliminators между подвыборкой и основным окном. |
| M4 | **Связь с апоптозом** | Апоптоз — конкурирующий риск (Fine–Gray в OS1-модели): late elimination vs apoptosis различать по CED-3::mKate2; известная апоптоз-карта эмбриона используется как априор. |
| M5 | **E-lineage** | Кишечная (E) линия элиминирует при постэмбриональной эндоредупликации (Lu & Roy 2014) — исключена из первичного анализа, отдельный вторичный анализ. |
| M6 | **Non-detection-дисклеймер** | «Не обнаружено элиминации в окне» ≠ «нет элиминации»; границы окна фиксируются в каждом отчёте. |
| M7 | **Структурная верификация** | На эндпоинтах подвыборки — Heidenhain iron haematoxylin (метод Бовери; `Celegans_Iron_Stain_Protocol.md`) для подтверждения, что GFP-позитивные структуры — центриоли, а не агрегаты. |

## 3. Окна по подсистемам (единая политика)
| Подсистема | Окно | Дисклеймер |
|---|---|---|
| OS1 (C. elegans) | ~100-cell → (подвыборка) комма-стадия+ | claim «в окне»; late eliminators флагаются |
| OS2 (RPE1) | первые сотни делений | горизонт не-детекта фиксируется (см. Immortalization §1.1a) |
| OS3 (абляция/трансплантация) | успех деления после манипуляции + время-до-события | survivor bias отчёт отдельно (см. `ARGUS_OS3_decision_tree.md`) |
| Immortalized (Hayflick) | ≥ N поколений или ≥ 6 мес., что раньше | «not detected in window» ≠ «no limit» (Immortalization §1.1a/§7) |

## 4. Верифицированные опоры
- Kalbfuss & Gönczy 2023a, *Sci Adv* `10.1126/sciadv.adg8682` (масштаб элиминации к комма-стадии)
- Lu & Roy 2014, *PLoS ONE* `10.1371/journal.pone.0110958` (E-lineage эндоредупликация)
- Sulston et al. 1983 (полный lineage; PMID 6684600) · Sulston & Horvitz 1977 (постэмбриональный; PMID 838129)
- Yao 2026 §7.2 (рамка snapshot/late eliminators/extension)

**Следующий шаг (для Джабы):** встроить M1–M7 в OS1-протокол (когда OS1-протокол C. elegans будет собираться) и зафиксировать долю подвыборки (M3) в пререгистрации OS1.
