# pi — BioSense

> ## 🔴 HARD RULE: MEMORY BEFORE ACTION
> 
> **Before any action (reading a file, running a command, converting, responding to user) — ALWAYS read:**
> 1. `_pi.md` — this file (rules, structure, commands)
> 2. `MAP.md` — project map (if exists)
> 3. `MEMORY.md` — decision history, prohibitions, tokens (if exists)
> 
> Violation → ignoring memory → loss of context → errors.
> 
> ---

**Parent:** LC  
**CONCEPT.md:** CONCEPT.md (if exists)  
**Created:** 2025-04-10

---

## Project Overview

BioSense is a biosensor data analysis platform that ingests real-time physiological signals (heart rate, temperature, galvanic skin response) from wearable devices, processes them through a machine learning pipeline, and outputs anomaly detection alerts and summary reports. The project aims to provide researchers and clinicians with a reliable, low-latency tool for monitoring patient health.

## Goals

- Achieve ≥95% accuracy in anomaly detection on test datasets.
- Maintain end-to-end pipeline latency under 500 ms per data batch.
- Generate human-readable reports in PDF and JSON formats.
- Ensure all components are containerized and deployable via Docker.

## Key Commands

| Command | Description |
|---------|-------------|
| `python src/run_pipeline.py` | Execute the full data ingestion, processing, and analysis pipeline |
| `python src/train_model.py` | Train the anomaly detection model on labeled data |
| `python src/generate_report.py --output report.pdf` | Generate a PDF report from the latest analysis results |
| `docker-compose up` | Start all services (ingestion, processing, database, web UI) |
| `pytest tests/` | Run unit and integration tests |

## Workflow

1. **Data Ingestion** — Collect raw sensor data from wearable devices via MQTT.  
   *Owner: Data Engineer*  
   *After Task A completion: Data Ingestion → Preprocessing*

2. **Preprocessing** — Clean, normalize, and segment the data into fixed‑length windows.  
   *Owner: Data Scientist*  
   *After Task B completion: Preprocessing → Feature Extraction*

3. **Feature Extraction** — Compute statistical and frequency‑domain features for each window.  
   *Owner: Data Scientist*  
   *After Task C completion: Feature Extraction → Anomaly Detection*

4. **Anomaly Detection** — Apply the trained ML model to classify windows as normal or anomalous.  
   *Owner: Lead Developer*  
   *After Task D completion: Anomaly Detection → Report Generation*

5. **Report Generation** — Compile results into a PDF/JSON report and store in the database.  
   *Owner: UI Designer*  
   *After Task E completion: Report Generation → Delivery*

6. **Delivery** — Push alerts to the dashboard and notify subscribed users via email.  
   *Owner: Project Manager*  
   *After Task F completion: Delivery → Monitoring*

## Dependencies

- **External Libraries:** `numpy`, `pandas`, `scikit-learn`, `tensorflow`, `paho-mqtt`, `reportlab`, `docker`
- **Data Sources:** Wearable device MQTT broker (host: `mqtt.biosense.local`, port 1883)
- **Database:** PostgreSQL instance for storing processed data and reports
- **Model Artifacts:** Pre‑trained model saved in `models/anomaly_detector.h5`
- **Configuration:** Environment variables defined in `.env` file (see `.env.example`)

## Exits

- **Completion Criteria:**
  - All workflow steps execute without errors on a test dataset of 10,000 samples.
  - Anomaly detection accuracy ≥95% on held‑out test set.
  - Pipeline latency <500 ms per batch (100 samples).
  - Reports are generated and stored in the `output/` directory.
- **Deliverables:**
  - Source code in `src/` with documentation.
  - Docker images for each service.
  - User manual (`docs/user_guide.md`).
  - Final project report (`docs/final_report.pdf`).

## Notes

- `MAP.md` and `MEMORY.md` are optional; if they exist, they must be read before any action.
- All changes to the pipeline must be reflected in `MEMORY.md` to maintain context.
- For questions, refer to `CONCEPT.md` or contact the project lead.