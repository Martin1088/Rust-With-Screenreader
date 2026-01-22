use anyhow::Result;
use brlapi::{Connection, text};

fn main() -> Result<()> {
    // 1. Verbindung zu BRLTTY/BrlAPI aufbauen
    let conn = Connection::open()
        .map_err(|e| anyhow::anyhow!("BrlAPI-Verbindung fehlgeschlagen: {e}"))?;

    // 2. Infos zur Anzeige abfragen (nur zum Debuggen)
    let (width, height) = conn
        .get_display_size()
        .map_err(|e| anyhow::anyhow!("Displaygröße konnte nicht ermittelt werden: {e}"))?;

    println!("Verbunden mit Braille-Display: {width}x{height} Zellen");

    // 3. Nachricht auf die Focus schreiben
    let msg = "Hallo Focus aus Rust!";

    // oneshot schickt den Text direkt zur Anzeige
    text::oneshot::write_message(&conn, msg)
        .map_err(|e| anyhow::anyhow!("Schreiben auf Brailleanzeige fehlgeschlagen: {e}"))?;

    println!("Nachricht gesendet: {msg}");

    Ok(())
}