//! Thermal capacitance (base unit joule per kelvin, kg · m² · s⁻² · K⁻¹).
//!
//! Thermal capacitance has the same kind as [temperature interval][ti], as this quantity relates
//! to change of temperature. Not of kind `TemperatureKind`, used by [thermodynamic
//! temperature][tt]. See [thermodynamic temperature][tt] for a full explanation.
//!
//! [ti]: ../temperature_interval/index.html
//! [tt]: ../thermodynamic_temperature/index.html

quantity! {
    /// Thermal capacitance (base unit joule per kelvin, kg · m² · s⁻² · K⁻¹).
    quantity: ThermalCapacitance; "thermal capacitance";
    /// Dimension of thermal capacitance, LM²T⁻²Th⁻¹ (base unit joule per kelvin, kg · m² · s⁻³
    /// · K⁻¹).
    dimension: ISQ<
        P2,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        N1,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram_meter_squared_per_second_squared_kelvin: prefix!(yotta) / prefix!(kilo);
            "Yg · m²/(s² · K)", "yottagram meter squared per second squared kelvin",
            "yottagrams meter squared per second squared kelvin";
        @zettagram_meter_squared_per_second_squared_kelvin: prefix!(zetta) / prefix!(kilo);
            "Zg · m²/(s² · K)", "zettagram meter squared per second squared kelvin",
            "zettagrams meter squared per second squared kelvin";
        @exagram_meter_squared_per_second_squared_kelvin: prefix!(exa) / prefix!(kilo); "Eg · m²/(s² · K)",
            "exagram meter squared per second squared kelvin",
            "exagrams meter squared per second squared kelvin";
        @petagram_meter_squared_per_second_squared_kelvin: prefix!(peta) / prefix!(kilo); "Pg · m²/(s² · K)",
            "petagram meter squared per second squared kelvin",
            "petagrams meter squared per second squared kelvin";
        @teragram_meter_squared_per_second_squared_kelvin: prefix!(tera) / prefix!(kilo); "Tg · m²/(s² · K)",
            "teragram meter squared per second squared kelvin",
            "teragrams meter squared per second squared kelvin";
        @gigagram_meter_squared_per_second_squared_kelvin: prefix!(giga) / prefix!(kilo); "Gg · m²/(s² · K)",
            "gigagram meter squared per second squared kelvin",
            "gigagrams meter squared per second squared kelvin";
        @megagram_meter_squared_per_second_squared_kelvin: prefix!(mega) / prefix!(kilo); "Mg · m²/(s² · K)",
            "megagram meter squared per second squared kelvin",
            "megagrams meter squared per second squared kelvin";
        /// Derived unit of thermal capacitance in base units. Equivalent to W/K.
        @kilogram_meter_squared_per_second_squared_kelvin: prefix!(kilo) / prefix!(kilo); "kg · m²/(s² · K)",
            "kilogram meter squared per second squared kelvin",
            "kilograms meter squared per second squared kelvin";
        @hectogram_meter_squared_per_second_squared_kelvin: prefix!(hecto) / prefix!(kilo);
            "hg · m²/(s² · K)", "hectogram meter squared per second squared kelvin",
            "hectograms meter squared per second squared kelvin";
        @decagram_meter_squared_per_second_squared_kelvin: prefix!(deca) / prefix!(kilo); "dag · m²/(s² · K)",
            "decagram meter squared per second squared kelvin",
            "decagrams meter squared per second squared kelvin";
        @gram_meter_squared_per_second_squared_kelvin: prefix!(none) / prefix!(kilo); "g · m/(s² · K)",
            "gram meter per second squared kelvin", "grams meter per second squared kelvin";
        @decigram_meter_squared_per_second_squared_kelvin: prefix!(deci) / prefix!(kilo); "dg · m²/(s² · K)",
            "decigram meter squared per second squared kelvin",
            "decigrams meter squared per second squared kelvin";
        @centigram_meter_squared_per_second_squared_kelvin: prefix!(centi) / prefix!(kilo);
            "cg · m²/(s² · K)", "centigram meter squared per second squared kelvin",
            "centigrams meter squared per second squared kelvin";
        @milligram_meter_squared_per_second_squared_kelvin: prefix!(milli) / prefix!(kilo);
            "mg · m²/(s² · K)", "milligram meter squared per second squared kelvin",
            "milligrams meter squared per second squared kelvin";
        @microgram_meter_squared_per_second_squared_kelvin: prefix!(micro) / prefix!(kilo); "µg · m/(s² · K)",
            "microgram meter squared per second squared kelvin",
            "micrograms meter squared per second squared kelvin";
        @nanogram_meter_squared_per_second_squared_kelvin: prefix!(nano) / prefix!(kilo); "ng · m²/(s² · K)",
            "nanogram meter squared per second squared kelvin",
            "nanograms meter squared per second squared kelvin";
        @picogram_meter_squared_per_second_squared_kelvin: prefix!(pico) / prefix!(kilo); "pg · m²/(s² · K)",
            "picogram meter squared per second squared kelvin",
            "picograms meter squared per second squared kelvin";
        @femtogram_meter_squared_per_second_squared_kelvin: prefix!(femto) / prefix!(kilo);
            "fg · m²/(s² · K)", "femtogram meter squared per second squared kelvin",
            "femtograms meter squared per second squared kelvin";
        @attogram_meter_squared_per_second_squared_kelvin: prefix!(atto) / prefix!(kilo); "ag · m²/(s² · K)",
            "attogram meter squared per second squared kelvin",
            "attograms meter squared per second squared kelvin";
        @zeptogram_meter_squared_per_second_squared_kelvin: prefix!(zepto) / prefix!(kilo);
            "zg · m²/(s² · K)", "zeptogram meter squared per second squared kelvin",
            "zeptograms meter squared per second squared kelvin";
        @yoctogram_meter_squared_per_second_squared_kelvin: prefix!(yocto) / prefix!(kilo);
            "yg · m²/(s² · K)", "yoctogram meter squared per second squared kelvin",
            "yoctograms meter squared per second squared kelvin";

        // Thermal capacitance is much more commonly expressed in terms of energy / temperature.
        @yottajoule_per_kelvin: prefix!(yotta); "YJ/K", "yottajoule per kelvin",
            "yottajoules per kelvin";
        @zettajoule_per_kelvin: prefix!(zetta); "ZJ/K", "zettajoule per kelvin",
            "zettajoules per kelvin";
        @exajoule_per_kelvin: prefix!(exa); "EJ/K", "exajoule per kelvin", "exajoules per kelvin";
        @petajoule_per_kelvin: prefix!(peta); "PJ/K", "petajoule per kelvin", "petajoules per kelvin";
        @terajoule_per_kelvin: prefix!(tera); "TJ/K", "terajoule per kelvin", "terajoules per kelvin";
        @gigajoule_per_kelvin: prefix!(giga); "GJ/K", "gigajoule per kelvin", "gigajoules per kelvin";
        @megajoule_per_kelvin: prefix!(mega); "MJ/K", "megajoule per kelvin", "megajoules per kelvin";
        @kilojoule_per_kelvin: prefix!(kilo); "kJ/K", "kilojoule per kelvin", "kilojoules per kelvin";
        @hectojoule_per_kelvin: prefix!(hecto); "hJ/K", "hectojoule per kelvin",
            "hectojoules per kelvin";
        @decajoule_per_kelvin: prefix!(deca); "daW/K", "decajoule per kelvin", "decajoules per kelvin";
        /// Derived unit of thermal capacitance in derived units. Equivalent to kg · m²/(s² · K).
        @joule_per_kelvin: prefix!(none); "J/K", "joule per kelvin", "joules per kelvin";
        @decijoule_per_kelvin: prefix!(deci); "dJ/K", "decijoule per kelvin", "decijoules per kelvin";
        @centijoule_per_kelvin: prefix!(centi); "cJ/K", "centijoule per kelvin",
            "centijoules per kelvin";
        @millijoule_per_kelvin: prefix!(milli); "mJ/K", "millijoule per kelvin",
            "millijoules per kelvin";
        @microjoule_per_kelvin: prefix!(micro); "µW/K", "microjoule per kelvin",
            "microjoules per kelvin";
        @nanojoule_per_kelvin: prefix!(nano); "nJ/K", "nanojoule per kelvin", "nanojoules per kelvin";
        @picojoule_per_kelvin: prefix!(pico); "pJ/K", "picojoule per kelvin", "picojoules per kelvin";
        @femtojoule_per_kelvin: prefix!(femto); "fJ/K", "femtojoule per kelvin",
            "femtojoules per kelvin";
        @attojoule_per_kelvin: prefix!(atto); "aJ/K", "attojoule per kelvin", "attojoules per kelvin";
        @zeptojoule_per_kelvin: prefix!(zepto); "zJ/K", "zeptojoule per kelvin",
            "zeptojoules per kelvin";
        @yoctojoule_per_kelvin: prefix!(yocto); "yJ/K", "yoctojoule per kelvin",
            "yoctojoules per kelvin";

        // Celsius for convenience.
        @kilogram_meter_squared_per_second_squared_degree_celsius: prefix!(kilo) / prefix!(kilo);
            "kg · m²/(s³ · °C)", "kilogram meter squared per second squared degree Celsius",
            "kilograms meter squared per second squared degree Celsius";
        @kilojoule_per_degree_celsius: prefix!(kilo); "kJ/°C", "kilojoule per degree Celsius",
            "kilojoules per degree Celsius";
        /// Derived unit of thermal capacitance in derived units. Equivalent to kg · m²/(s² · K).
        @joule_per_meter_degree_celsius: prefix!(none); "J/°C", "joule per degree Celsius",
            "joules per degree Celsius";
        @millijoule_per_degree_celsius: prefix!(milli); "mJ/°C", "millijoule per degree Celsius",
            "millijoules per degree Celsius";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use crate::num::One;
        use crate::si::length as l;
        use crate::si::mass as m;
        use crate::si::energy as e;
        use crate::si::quantities::*;
        use crate::si::temperature_interval as ti;
        use crate::si::thermal_capacitance as tc;
        use crate::si::time as t;
        use crate::tests::Test;

        #[test]
        fn check_dimension() {
            let _: ThermalCapacitance<V> = Mass::new::<m::kilogram>(V::one())
                * Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one())
                / (Time::new::<t::second>(V::one())
                    * Time::new::<t::second>(V::one())
                    * TemperatureInterval::new::<ti::kelvin>(V::one()));
        }

        #[test]
        fn check_base_units() {
            test::<m::yottagram, ti::kelvin, tc::yottagram_meter_squared_per_second_squared_kelvin>();
            test::<m::zettagram, ti::kelvin, tc::zettagram_meter_squared_per_second_squared_kelvin>();
            test::<m::exagram, ti::kelvin, tc::exagram_meter_squared_per_second_squared_kelvin>();
            test::<m::petagram, ti::kelvin, tc::petagram_meter_squared_per_second_squared_kelvin>();
            test::<m::teragram, ti::kelvin, tc::teragram_meter_squared_per_second_squared_kelvin>();
            test::<m::gigagram, ti::kelvin, tc::gigagram_meter_squared_per_second_squared_kelvin>();
            test::<m::megagram, ti::kelvin, tc::megagram_meter_squared_per_second_squared_kelvin>();
            test::<m::kilogram, ti::kelvin, tc::kilogram_meter_squared_per_second_squared_kelvin>();
            test::<m::hectogram, ti::kelvin, tc::hectogram_meter_squared_per_second_squared_kelvin>();
            test::<m::decagram, ti::kelvin, tc::decagram_meter_squared_per_second_squared_kelvin>();
            test::<m::gram, ti::kelvin, tc::gram_meter_squared_per_second_squared_kelvin>();
            test::<m::decigram, ti::kelvin, tc::decigram_meter_squared_per_second_squared_kelvin>();
            test::<m::centigram, ti::kelvin, tc::centigram_meter_squared_per_second_squared_kelvin>();
            test::<m::milligram, ti::kelvin, tc::milligram_meter_squared_per_second_squared_kelvin>();
            test::<m::microgram, ti::kelvin, tc::microgram_meter_squared_per_second_squared_kelvin>();
            test::<m::nanogram, ti::kelvin, tc::nanogram_meter_squared_per_second_squared_kelvin>();
            test::<m::picogram, ti::kelvin, tc::picogram_meter_squared_per_second_squared_kelvin>();
            test::<m::femtogram, ti::kelvin, tc::femtogram_meter_squared_per_second_squared_kelvin>();
            test::<m::attogram, ti::kelvin, tc::attogram_meter_squared_per_second_squared_kelvin>();
            test::<m::zeptogram, ti::kelvin, tc::zeptogram_meter_squared_per_second_squared_kelvin>();
            test::<m::yoctogram, ti::kelvin, tc::yoctogram_meter_squared_per_second_squared_kelvin>();

            test::<m::kilogram, ti::degree_celsius,
                tc::kilogram_meter_squared_per_second_squared_degree_celsius>();

            fn test<
                M: m::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalCapacitance::new::<TC>(V::one()),
                    &(Mass::new::<M>(V::one())
                        * Length::new::<l::meter>(V::one()) * Length::new::<l::meter>(V::one())
                        / (Time::new::<t::second>(V::one())
                            * Time::new::<t::second>(V::one())
                            * TemperatureInterval::new::<TI>(V::one()))));
            }
        }

        #[test]
        fn check_power_per_length_ti_units() {
            test::<e::yottajoule, l::meter, ti::kelvin, tc::yottajoule_per_kelvin>();
            test::<e::zettajoule, l::meter, ti::kelvin, tc::zettajoule_per_kelvin>();
            test::<e::exajoule, l::meter, ti::kelvin, tc::exajoule_per_kelvin>();
            test::<e::petajoule, l::meter, ti::kelvin, tc::petajoule_per_kelvin>();
            test::<e::terajoule, l::meter, ti::kelvin, tc::terajoule_per_kelvin>();
            test::<e::gigajoule, l::meter, ti::kelvin, tc::gigajoule_per_kelvin>();
            test::<e::megajoule, l::meter, ti::kelvin, tc::megajoule_per_kelvin>();
            test::<e::kilojoule, l::meter, ti::kelvin, tc::kilojoule_per_kelvin>();
            test::<e::hectojoule, l::meter, ti::kelvin, tc::hectojoule_per_kelvin>();
            test::<e::decajoule, l::meter, ti::kelvin, tc::decajoule_per_kelvin>();
            test::<e::joule, l::meter, ti::kelvin, tc::joule_per_kelvin>();
            test::<e::decijoule, l::meter, ti::kelvin, tc::decijoule_per_kelvin>();
            test::<e::centijoule, l::meter, ti::kelvin, tc::centijoule_per_kelvin>();
            test::<e::millijoule, l::meter, ti::kelvin, tc::millijoule_per_kelvin>();
            test::<e::microjoule, l::meter, ti::kelvin, tc::microjoule_per_kelvin>();
            test::<e::nanojoule, l::meter, ti::kelvin, tc::nanojoule_per_kelvin>();
            test::<e::picojoule, l::meter, ti::kelvin, tc::picojoule_per_kelvin>();
            test::<e::femtojoule, l::meter, ti::kelvin, tc::femtojoule_per_kelvin>();
            test::<e::attojoule, l::meter, ti::kelvin, tc::attojoule_per_kelvin>();
            test::<e::zeptojoule, l::meter, ti::kelvin, tc::zeptojoule_per_kelvin>();
            test::<e::yoctojoule, l::meter, ti::kelvin, tc::yoctojoule_per_kelvin>();

            test::<e::kilojoule, l::meter, ti::degree_celsius, tc::kilojoule_per_degree_celsius>();
            test::<e::joule, l::meter, ti::degree_celsius, tc::joule_per_meter_degree_celsius>();
            test::<e::millijoule, l::meter, ti::degree_celsius, tc::millijoule_per_degree_celsius>();

            fn test<
                E: e::Conversion<V>,
                L: l::Conversion<V>,
                TI: ti::Conversion<V>,
                TC: tc::Conversion<V>>()
            {
                Test::assert_approx_eq(&ThermalCapacitance::new::<TC>(V::one()),
                    &(Energy::new::<E>(V::one())
                        / TemperatureInterval::new::<TI>(V::one())));
            }
        }
    }
}
