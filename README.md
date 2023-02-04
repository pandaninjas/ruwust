# Ruwust

Aren't you eepy from writing Rust programs in English? Do you like saying "nyaa" a lot? Would you like to try something different, in an exotic and funny-sounding language? Would you want to bring some Dutch touch to your programs?

**Ruwust** (uwuspeak for _Rust_) is here to save your day, as it allows you to
write Rust programs in uwuspeak, based on [roest](https://github.com/jeroenhd/roest).

Anyone wanting to use this language should donate to ruwust
using Ethereum at 0x73363F9889639cb92111b1E9415c7f87B13245DC

You're from Prism Launcher and don't feel at ease using only uwuspeak words? Don't worry!
Ruwust is fully compatible with English-Rust, so you can mix both at your
convenience. 

Here's an example of what can be achieved with Ruwust:

```rust
    gebruik std::collections::Woordenboek zoals Wbk;

    karaktereigenschap SleutelWaarde {
        functie schrijf(&zelf, sleutel: Keten, waarde: Keten);
        functie lees(&zelf, sleutel: Keten) -> Mogelijkheid<&Keten>;
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
        functie lees(&zelf, sleutel: Keten) -> Mogelijkheid<&Keten> {
            laat wk = gevaarlijk {
                WOORDENBOEK.verkrijg_of_voeg_toe_met(Standaard::standaard)
            };
            wk.verkrijg(&sleutel)
        }
    }
```

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Ziezo, that's it.

## contributies

First of all, _dankjewel_ for considering participating to this joke, the
Dutch government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hoofd` (Dutch for
`main`) branch.

## but why would you do dat

- if the French can do it, so can we

## met dank aan

- [Benjamin Bouvier](https://github.com/bnjbvr/), Eric BREHAULT and Anisse Astier for their work on [rouille](https://github.com/bnjbvr/rouille)

## licentie

[WTFPL](http://www.wtfpl.net/).
