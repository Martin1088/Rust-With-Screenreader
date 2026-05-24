# Temporal
Plus ein Browser-Tab auf http://localhost:8233 (Temporal UI).

## Start-Sequenz

Terminal A — Temporal Server
```
temporal server start-dev
```
Warte bis “Server: …started” steht. UI auf :8233 ist jetzt erreichbar.

Terminal B — Worker
```
cargo run --bin worker
```
Du siehst etwas wie “Starte Prähab-Worker auf Task-Queue ‘prehab-queue’”. Im UI unter localhost:8233/namespaces/default/task-queues/prehab-queue siehst du jetzt “1 Worker” — schöner Demo-Moment, das vorab zu zeigen.

Terminal C — API
```
cargo run --bin api
```
“HTTP API auf http://localhost:9000” — Server bereit für curl.

## Einschreibung und Sichtbarkeit
```
curl -X POST http://localhost:9000/enroll \
-H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "surgery_date": "2026-06-15T08:00:00Z"}'
```
Wechsel zum Browser auf localhost:8233:

	•	Klick auf “Workflows” → der prehab-PSN-DEMO-001 ist da
	•	Klick rein → zeig die Event-History
	•	“Workflow Started”, “Activity scheduled (PROM-Anfrage)”, “Activity completed”, “Timer Started” (Wartet auf PROM-Antwort)


## Signal trifft Workflow
```
curl -X POST http://localhost:9000/prom \
-H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "baseline", "score": 42}'
```

Browser refresh:

	•	Neuer History-Eintrag “Signal received: prom_completed”
	•	Timer “Started” → Workflow schläft jetzt 7 Tage bis zum ersten wöchentlichen Check-in

Worker-Terminal (B): dort siehst du im Log: PROM-Signal empfangen, dann ✅ PROM rechtzeitig empfangen.

Terminal B (Worker): Drück Strg+C.


Terminal B: Starte neu:
```
cargo run --bin worker
```

## Wöchentliche Check-ins (Tempo)
```
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "weekly_1", "score": 50}'
```
```
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "weekly_2", "score": 55}'
```
## Human-in-the-Loop
```
curl -X POST http://localhost:9000/prom -H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "questionnaire": "final_assessment", "score": 25}'
```

Worker-Log (Terminal B):

- Kritischer Score — warte auf Koordinator-Review
- Studienkoordinator informiert
- Browser: Workflow wartet jetzt auf coordinator_review-Signal. Zeig in der UI: “Pending Signals” oder die “Awaiting” History.
```
curl -X POST http://localhost:9000/coordinator-review \
-H "Content-Type: application/json" \
-d '{"pseudonym": "PSN-DEMO-001", "proceed": false, "note": "Score zu niedrig, OP verschoben"}'
```

Worker-Log:  Koordinator-Entscheidung empfangen

Browser: Workflow endet mit Status “Completed” und Result PrehabResult::SurgeryDeferred. Klick die History durch — der ganze Pfad ist nachvollziehbar.