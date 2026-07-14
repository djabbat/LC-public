# Письмо Алексею Филиппову — OpenFlexure v6.1.5 (прямой микроскоп)

**Дата:** 2026-07-14
**Кому:** Алексей Филиппов (инженерный партнёр ARGUS)
**От:** Джаба Ткемаладзе
**Контекст:** Ошибочно отправлено в тред Fraunhofer IVV (Max Hesse). Max ответил: «I think you've sent this to the wrong contacts 😉»

---

Алексей, привет!

Вчера в разговоре с инженером-ворчуном пришло важное инженерное прозрение по ARGUS — делюсь сразу.

**Прямая схема вместо инвертированной.** Текущий дизайн — инвертированный: объектив 60×/1.2 Water снизу, вода в муфте с капиллярной подачей. Проблема: вода в инвертированном положении стремится вниз. Инженер предложил усложнить манипулирование, а также фокусирование объектива и облегчить оптомеханику: прямой микроскоп. Объектив сверху → вода держится гравитацией. Лазер абляции — снизу, через стеклянное дно, манипуляторы сбоку.

**OpenFlexure даёт готовую механику.** Вы уже обсуждали OpenFlexure v7 с вашим инженером. Инженер-ворчун сейчас смотрит STL v6.1.5: RMS-оптика, флексурный столик <100 нм, моторизованный, RasPi Camera. Цена печати + оптика + RasPi = ~$300–500. Вся механика уже спроектирована, протестирована и откалибрована. Это $300 вместо $2,045 за V1.

**Предлагаю:** вместо custom-сборки инвертированного микроскопа — напечатать OpenFlexure, добавить герметичный бокс (CO₂/O₂/37°C) и проверить: ловит ли деление BJ-hTERT с 40× сухим. Если да → тестируем водяную иммерсию 60× сверху (вода не падает) → потом лазер снизу. V1 за 2–3 недели вместо месяцев.

Что думаете?

**Ссылки на STL:**
- https://build.openflexure.org/openflexure-microscope/v6.1.5/optics_picamera_2_rms_f50d13.stl
- https://microscope-stls.openflexure.org/#/v6.1.5?enable_smart_brim=true&reflection_illumination=false&optics=rms_f50d13&camera=picamera_2&use_pilens_optics_module=false&riser=sample&microscope_stand%3Abox_h=30&pi_in_base=true&base=bucket&legacy_picamera_tools=false&include_actuator_tension_band=false&include_actuator_drilling_jig=false&motorised=true&use_motor_gears_for_hand_actuation=false

С уважением, Джаба
