use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <form>
                <div class={classes!("graydient")}>
                    <h1>
                        {"Migrationsrechtlicher Prozesskostenrechner (Stand 2025)"}
                    </h1>
                    <p  class={classes!("mb-0")}>
                        {"Erstellt von "}
                        <a href="https://aufentha.lt">{"Marcel Keienborg"}</a>
                        {". Bitte beachte unbedingt auch die Hinweise unten auf dieser Seite."}
                    </p>
                </div>
                <div class={classes!("graydient")}>
                    <h2>
                        {"Streitwerte"}
                    </h2>
                    <p>
                        {"Grundlage für die Berechnung der Gebühren ist der sogenannte Streitwert. Wir müssen also zuerst die Streitwerte für deine Angelegenheit ermitteln. Plural, weil gerade in gerichtlichen Verfahren häufig zusätzlich zur Klage noch ein Antrag auf Gewährung vorläufigen Rechtsschutzes erforderlich ist. Manchmal wird auch nur ein Antrag auf vorläufigen Rechtsschutz gestellt. Deswegen kann man hier verschiedene Optionen wählen. Außerdem hängt die konkrete Höhe des Streitwerts auch von der Anzahl der Personen ab, die in einem Verfahren zusammengefasst werden."}
                    </p>
                    <p>
                        <label for="verfahren">{"Wähle aus, ob die Gebühren nur für ein Hauptsacheverfahren, nur für dein Verfahren
                        zum vorläufigen Rechtsschutz, oder für beides berechnet werden sollen."}</label>
                    </p>
                    <p>
                        <select aria-label="Auswahl der Verfahrensart" id="verfahren">
                            <option value="0" selected=true>{"Nur Hauptsacheverfahren"}</option>
                            <option value="1">{"Nur Verfahren zum vorläufigen Rechtsschutz"}</option>
                            <option value="2">{"Hauptsacheverfahren und Verfahren zum vorläufigen Rechtsschutz"}</option>
                        </select>
                    </p>
                    <p>
                        <label for="thema">{"Wähle ein Thema, dann versucht der Rechner, dir die passenden Streitwerte vorzuschlagen.
                        Du kannst aber auch manuell selbst Streitwerte angeben."}</label>
                    </p>
                    <p>
                        <select aria-label="Auswahl des Themas" id="thema">
                            <option value="0">{"Asylrecht: Zulässigkeit (z.B. Dublin, Drittstaatenfall, Folgeantrag)"}</option>
                            <option value="1">{"Asylrecht: Anerkennungsverfahren"}</option>
                            <option value="2">{"Asylrecht: Widerruf/Rücknahme"}</option>
                            <option value="3">{"Asylrecht: Untätigkeitsklage"}</option>
                            <option value="4" selected=true>{"Aufenthaltsrecht: Aufenthaltstitel inkl. Untätigkeitsklage"}</option>
                            <option value="5">{"Ausweisung"}</option>
                            <option value="6">{"Pass/Passersatz"}</option>
                            <option value="7">{"Aufenthaltsrecht: Duldung und Abschiebung inkl. Ausbildungs-/Beschäftigungsduldung,
                                Untätigkeitsklage"}</option>
                            <option value="8">{"Einbürgerung und Feststellung der Staatsangehörigkeit"}</option>
                        </select>
                    </p>
                //     <div>
                //         <div>{"Anzahl Personen"}</div>
                //         <div>{"Streitwerte"}</div>
                //         <div>{"Wertgebühr (§ 13 RVG)"}</div>
                //         <div>{"Wertgebühr (§ 49 RVG / Prozesskostenhilfe)"}</div>
                //         <div>{"Wertgebühr (GKG)"}</div>
                //         // new row
                //         <div></div>
                //         <div>{"Hauptsache"}</div>
                //         <div></div>
                //         <div></div>
                //         <div></div>                  
                //     </div>
                </div>
                <div class={classes!("graydient")}>
                    <h2>
                        {"Rechtliche Hinweise"}
                    </h2>
                    <p>
                        {"Dieser Prozesskostenrechner berechnet gesetzliche Gebühren auf der Grundlage des 
                        Rechtsanwaltsvergütungsgesetzes ("}
                        <a href="https://dejure.org/gesetze/RVG">{"RVG"}</a>
                        {"), des Gerichtskostengesetzes ("}
                        <a href="https://dejure.org/gesetze/GKG">{"GKG"}</a>
                        {"), des "}
                        <a href="https://www.bverwg.de/rechtsprechung/streitwertkatalog">{"Streitwertkatalogs"}</a>
                        {" des Bundesverwaltungsgerichts und meiner Erfahrung mit der Interpretation dieser Vorgaben durch die
                        Verwaltungsgerichte vornehmlich in NRW. Der Rechner dient nur einer unverbindlichen
                        Orientierung und kann eine fachkundige Beratung nicht ersetzen. Seine Nutzung erfolgt insofern auf eigene
                        Gefahr. Es kann eine Vielzahl von Gründen geben, warum ein Gericht höhere oder niedrigere Kosten festsetzt,
                        als von diesem Rechner ermittelt. Auch möchten viele Rechtsanwält*innen Vergütungsvereinbarungen schließen,
                        die zum Teil deutlich von den gesetzlich vorgesehenen Gebühren abweichen. Der Rechner geht zudem auch davon
                        aus, dass sich die Behördenseite nicht anwaltlich vertreten lässt. Tatsächlich lassen sich Behörden in
                        migrationsrechtlichen Streitigkeiten erfahungsgemäß selten anwaltlich vertreten. Völlig ausgeschlossen ist
                        es aber auch nicht. Ggf. würden hierdurch weitere Kosten entstehen, die dieser Rechner nicht berücksichtigt.
                        Außerdem berücksichtigt dieser Rechner keine Gebühren, die durch Behörden im Verwaltungsverfahren erhoben
                        werden. Widerspruchs- und Remonstrationsverfahren werden ebenfalls (noch?) nicht abgebildet, was vor allem
                        daran liegt, dass ich hauptsächlich in NRW tätig bin, wo es kaum noch Widerspruchsverfahren gibt. Die
                        Anrechnung der Geschäftsgebühr auf die Verfahrensgebühr (Vorbemerkung 4 zu Teil 3 VV RVG) erfolgt immer auf
                        die 1. Instanz des Hauptsacheverfahrens, da dies auch in der Praxis nahezu immer der Fall sein wird. Soweit
                        zumindest theoretisch auch Fälle konstruiert werden können, in denen die Anrechnung auf die Verfahrensgebühr
                        in einer höheren Insatz erfolgt, bleiben diese Fälle hier um der Einfachheit willen unberücksichtigt."}
                    </p>
                    <p>
                        {"Der Rechner geht äußerst sparsam mit deinen Daten um. Zwar werden einige technisch benötigte Daten,
                        insbesondere deine IP-Adresse, an meinen Server gesendet und von meinem Server verarbeitet. Das ist nötig,
                        um die zum Rechner gehörenden Dateien an deinen Browser oder dein sonstiges Gerät, mit welchem du den
                        Rechner ausführst, schicken zu können. Der Rechner wird aber vollständig auf deinem Gerät ausgeführt. Das
                        bedeutet, dass alle Daten, die du in den Rechner eingibst, und die Ergebnisse, die von meinem Rechner
                        erzeugt werden, vollständig bei dir verbleiben, und erst gar nicht an meinen Server geschickt, geschweige
                        denn verarbeitet oder gespeichert werden."}
                    </p>
                    <p>
                        {"Der Rechner ist zudem auch als Freie Software unter den Lizenzen Apache, Version 2.0, und MIT
                        veröffentlicht. Du kannst dir die Software also auch aus dem "}
                        <a href="https://github.com/dusmarcel/mpkr25">{"Repository"}</a>
                        {" herunterladen und sie dann ganz auf einem Gerät deiner Wahl ausführen. In diesem Falle hast du mit meinem Server gar nichts mehr zu tun, und die
                        Notwendigkeit, Daten an meinen Server zu übertragen, entfällt ganz."}
                    </p>
                    <p>
                        {"Und schließlich geht es hier noch zu meinem „"}
                        <a href="https://aufentha.lt/index.php/impressum/">{"Impressum"}</a>
                        {"“."}
                    </p>
                </div>
            </form>
            <noscript>{"This page contains webassembly and javascript content, please enable javascript in your browser."}</noscript>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
