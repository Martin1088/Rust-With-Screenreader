# 1. Patient einschreiben → Workflow startet
curl -X POST http://localhost:9000/enroll \
-H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "surgery_date": "2026-06-15T08:00:00Z"}'

# UI öffnen: http://localhost:8233 → Workflow läuft, schläft auf Baseline-PROM

# 2. PROM-Antwort eintreffen lassen
curl -X POST http://localhost:9000/prom \
-H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "baseline", "score": 42}'

# 3. KILLER-MOMENT: Worker killen (Strg+C in Terminal B)
# → Workflow bleibt sichtbar in UI, wartet weiter
# Worker neu starten:
cargo run --bin worker
# → Workflow läuft nahtlos weiter

# 4. Weitere PROMs (wöchentliche Check-ins)
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "weekly_1", "score": 50}'
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "weekly_2", "score": 55}'
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "weekly_3", "score": 58}'

# 5. Final Assessment mit KRITISCHEM Wert (< 30) → triggert Review
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "final_assessment", "score": 25}'

# UI zeigt: Workflow wartet auf coordinator_review-Signal

# 6. Human-in-the-Loop: Koordinator entscheidet
curl -X POST http://localhost:9000/coordinator-review -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "proceed": false, "note": "Score zu niedrig, OP verschoben"}'

# Workflow endet mit PrehabResult::SurgeryDeferred
