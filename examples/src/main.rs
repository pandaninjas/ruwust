roest::roest! {
    extern krat roest;

    gebruik std::collections::Woordenboek zoals Wbk;

    karaktereigenschap SleutelWaarde {
        functie schrijf(&zelf, sleutel: Keten, waarde: Keten);
        functie lees(&zelf, sleutel: Keten) -> Resultaat<Mogelijkheid<&Keten>, Keten>;
    }

    vast veranderlijk WOORDENBOEK: Mogelijkheid<Wbk<Keten, Keten>> = Geen;

    structuur Concreet;

    uitwerking SleutelWaarde voor Concreet {
        functie schrijf(&zelf, sleutel: Keten, waarde: Keten) {
            laat wk = gevaarlijk {
                WOORDENBOEK.verkrijg_of_voeg_toe_met(Standaard::standaard)
            };
            wk.voeg_in(sleutel, waarde);
        }
        functie lees(&zelf, sleutel: Keten) -> Resultaat<Mogelijkheid<&Keten>, Keten> {
            // laat wk = gevaarlijk {
            //     WOORDENBOEK.verkrijg_of_voeg_toe_met(Standaard::standaard)
            // };
            // wk.verkrijg(&sleutel)
            als laat Enige(wbk) = gevaarlijk { WOORDENBOEK.als_verw() } {
                Goed(wbk.verkrijg(&sleutel))
            } anders {
                Ft("ophalen uit woordenboek".tot())
            }
        }
    }

    openbaar(krat) functie misschien(i: u32) -> Mogelijkheid<Resultaat<u32, Keten>> {
        als i % 2 == 1 {
            als i == 42 {
                Enige(Ft(Keten::van("poep")))
            } anders {
                Enige(Goed(33))
            }
        } anders {
            Geen
        }
    }

    gelijktijdige functie voorbeeld() {
    }

    gelijktijdige functie voorbeeld2() {
        voorbeeld().wacht_af;
    }

    functie hoofd() {
        laat veranderlijk x = 31;

        gelijkend x {
            42 => {
                schrijfrgl!("pannekoek")
            }
            _ => schrijfrgl!("zie daar")
        }

        voor i binnen 0..10 {
            laat val = lus {
                ontsnap i;
            };

            zolang x < val {
                x += 1;
            }

            x = als laat Enige(resultaat) = misschien(i) {
                resultaat.pak_uit()
            } anders {
                12
            };
        }
    }
}
