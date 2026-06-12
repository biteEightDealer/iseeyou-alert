# I See You Alerts

**I See You Alerts** is a hardened security and auditing tool for Linux servers. It runs natively as a background service (`daemon`), continuously monitoring and controlling authentication events over port 22 (SSH) in real time.

Whenever an authentication event occurs (whether successful or failed), the tool processes the session metadata and dispatches a **custom JSON alert** to any remote HTTP endpoint (such as Webhooks, SIEM systems, Slack/Discord channels, or custom APIs).

---

## Key Features

* **Background Monitoring (Native Daemon):** Continuous, low-overhead execution managed directly by `systemd`.
* **Real-Time Alerting:** Immediate notification upon legitimate logins or active brute-force attacks on port 22.
* **Flexible JSON Payload:** Sends a structured object containing critical operational data: source IP, username, authentication status, and timestamps.
* **Idempotent Deployment:** Automated Bash installer that allows updating the tool without blowing away existing configurations.
* **Extensible Design (Fail2ban-inspired):** Implements split configuration (base vs. local) to ensure safe software updates.

---

## System Architecture

The tool integrates natively with system logging facilities and is managed as a standard OS service: