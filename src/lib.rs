// SPDX-License-Identifier: LGPL-3.0-or-later
//
// Copyright © 2026 Jaxydog
//
// This file is part of Fig.
//
// Fig is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// Fig is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Fig. If not,
// see <https://www.gnu.org/licenses/>.

//! Provides a simple API for declaring custom `cfg` predicates at compile-time.

use std::env::VarError;

/// Converts the given list of strings into a valid value string.
fn list_to_value_str(values: &[&str]) -> Box<str> {
    const SEPARATOR: &str = ", ";

    let final_index = values.len().saturating_sub(1);
    let initial_capcity = values.iter().map(|s| s.len()).sum::<usize>() + (SEPARATOR.len() * final_index);
    let mut value_string = String::with_capacity(initial_capcity);

    for (index, value) in values.iter().enumerate() {
        value_string.push('"');
        value_string += value;
        value_string.push('"');

        if index != final_index {
            value_string += SEPARATOR;
        }
    }

    value_string.into_boxed_str()
}

/// A custom configuration value.
#[must_use = "this value does nothing unless used"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cfg<'s> {
    /// The configuration key. This must be a valid identifier.
    key: &'s str,
}

impl<'i> Cfg<'i> {
    /// Creates a new [`Cfg`] entry.
    pub const fn new(key: &'i str) -> Self {
        Self { key }
    }

    /// Declares that this configuration is not assigned any values and register it.
    #[must_use = "this value does nothing unless used"]
    pub fn assigned_none(self) -> impl CheckedCfg<'i> {
        struct Impl<'i>(&'i str);

        impl<'i> CheckedCfg<'i> for Impl<'i> {
            fn key(&self) -> &'i str {
                self.0
            }

            fn is_assignable(&self, value: Option<&str>) -> bool {
                value.is_none()
            }
        }

        println!("cargo::rustc-check-cfg=cfg({}, values(none()))", self.key);

        Impl(self.key)
    }

    /// Declares that this configuration is not assigned any values and register it.
    #[must_use = "this value does nothing unless used"]
    pub fn assigned_any(self) -> impl CheckedCfg<'i> {
        struct Impl<'i>(&'i str);

        impl<'i> CheckedCfg<'i> for Impl<'i> {
            fn key(&self) -> &'i str {
                self.0
            }

            fn is_assignable(&self, value: Option<&str>) -> bool {
                value.is_some()
            }
        }

        println!("cargo::rustc-check-cfg=cfg({}, values(any()))", self.key);

        Impl(self.key)
    }

    /// Declares that this configuration is not assigned any values and register it.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided list is empty.
    #[must_use = "this value does nothing unless used"]
    pub fn assigned_one_of(self, values: &'i [&'i str]) -> impl CheckedCfg<'i> {
        struct Impl<'i>(&'i str, &'i [&'i str]);

        impl<'i> CheckedCfg<'i> for Impl<'i> {
            fn key(&self) -> &'i str {
                self.0
            }

            fn is_assignable(&self, value: Option<&str>) -> bool {
                value.is_some_and(|v| self.1.contains(&v))
            }
        }

        assert!(!values.is_empty(), "at least one value should be provided");

        println!("cargo::rustc-check-cfg=cfg({}, values({}))", self.key, self::list_to_value_str(values));

        Impl(self.key, values)
    }

    /// Declares that this configuration is not assigned any values and register it.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided list is empty.
    #[must_use = "this value does nothing unless used"]
    pub fn assigned_none_or_one_of(self, values: &'i [&'i str]) -> impl CheckedCfg<'i> {
        struct Impl<'i>(&'i str, &'i [&'i str]);

        impl<'i> CheckedCfg<'i> for Impl<'i> {
            fn key(&self) -> &'i str {
                self.0
            }

            fn is_assignable(&self, value: Option<&str>) -> bool {
                value.is_none_or(|v| self.1.contains(&v))
            }
        }

        assert!(!values.is_empty(), "at least one value should be provided");

        println!("cargo::rustc-check-cfg=cfg({}, values(none(), {}))", self.key, self::list_to_value_str(values));

        Impl(self.key, values)
    }
}

/// A custom configuration value that is being checked and can be set.
pub trait CheckedCfg<'s> {
    /// Returns the key used by this configuration.
    fn key(&self) -> &'s str;

    /// Returns `true` if the value can be assigned to this configuration.
    fn is_assignable(&self, value: Option<&'_ str>) -> bool;

    /// Sets the configuration for the current build.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided value is not assignable to the configuration.
    fn set(&self, value: Option<&'_ str>) {
        assert!(self.is_assignable(value), "`{value:?}` is not assignable to configuration '{}'", self.key());

        if let Some(value) = value {
            println!(r#"cargo::rustc-cfg={}="{value}""#, self.key());
        } else {
            println!("cargo::rustc-cfg={}", self.key());
        }
    }

    /// Sets the configuration for the current build from the given environment variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided value is not assignable to the configuration, or the given key contains
    /// an invalid character.
    fn set_from_env(&self, variable_key: &str) {
        match std::env::var(variable_key) {
            Ok(value) if !value.is_empty() => self.set(Some(&value)),
            Ok(_) | Err(VarError::NotPresent) => self.set(None),
            Err(error) => panic!("{error}"),
        }
    }

    /// Sets the configuration for the current build from the given environment variable.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided value is not assignable to the configuration, or the given key contains
    /// an invalid character.
    fn set_from_env_or_else<D>(&self, variable_key: &str, default: D)
    where
        D: FnOnce() -> Option<String>,
    {
        match std::env::var(variable_key) {
            Ok(value) if !value.is_empty() => self.set(Some(&value)),
            Ok(_) | Err(VarError::NotPresent) => self.set(default().as_deref()),
            Err(error) => panic!("{error}"),
        }
    }
}
