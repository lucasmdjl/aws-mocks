/*
 * aws_mock - A mocking library for AWS.
 *
 * Copyright (C) 2024 Lucas M. de Jong Larrarte
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fs;

fn main() {
    let paths = fs::read_dir("./resources").unwrap();
    let paths = paths.map(|path| path.unwrap().file_name().into_string().unwrap()).collect::<Vec<_>>();
    generate_files(&paths.iter().map(String::as_str).collect::<Vec<_>>());
}

fn generate_files(names: &[&str]) {
    generate_lib_file(names);
    for name in names {
        generate_file(name);
    }
}

fn generate_lib_file(names: &[&str]) {
    fs::write("./src/lib.rs", generate_lib(names)).unwrap()
}

fn generate_file(name: &str) {
    let methods = read_method_names(name);
    let methods = methods.iter().map(String::as_str).collect::<Vec<_>>();
    fs::write(format!("./src/{}.rs", camel_to_snake(name)), generate(name, &methods)).unwrap()
}

fn generate_lib(names: &[&str]) -> String {
    format!("{copyright}\n{mods}\n", copyright = create_copyritgh(), mods = create_mods(names))
}

fn read_method_names(name: &str) -> Vec<String> {
    let names = fs::read_to_string(format!("./resources/{name}")).unwrap();
    get_method_names(names)
}

fn create_mods(names: &[&str]) -> String {
    names.iter().map(gen_mod).collect::<Vec<_>>().join("\n") + "\n"
}

fn generate(name: &str, methods: &[&str]) -> String {
    format!(
        "{copyright}\n{import}\n{struct}\n{impl}\n{trait}\n{trait_impl}\n{trait_impl_borrow}\n{mock}\n",
        copyright = create_copyritgh(),
        import = create_imports(name, methods),
        struct = create_struct(name),
        impl = create_impl(name),
        trait = create_trait(name, methods),
        trait_impl = create_trait_impl(name, methods),
        trait_impl_borrow = create_trait_borrow_impl(name, methods),
        mock = create_mock(name, methods),
    )
}

fn gen_mod(name: &&str) -> String {
    let snake = camel_to_snake(name);
    let kebab = snake.replace('_', "-");
    format!(
        "#[cfg(feature=\"{kebab}\")]
pub mod {snake};"
    )
}
fn get_method_names(names: String) -> Vec<String> {
    names.split_whitespace().map(|name| name.replace('-', "_")).collect::<Vec<_>>()
}

fn create_copyritgh() -> String {
    "/*
 * aws_mock - A mocking library for AWS.
 *
 * Copyright (C) 2024 Lucas M. de Jong Larrarte
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */".to_string()
}

fn create_imports(name: &str, methods: &[&str]) -> String {
    let pack = format!("aws_sdk_{}", name.to_ascii_lowercase());
    methods.iter().map(|method| {
        format!(
"use {pack}::operation::{method}::{{builders::*, *}};")
    }).collect::<Vec<_>>().join("\n") + &format!("
use {pack}::error::SdkError;
use std::future::Future;
use aws_config::SdkConfig;
use {pack}::Client;

pub use {pack}::*;
"
    )
}

fn create_struct(name: &str) -> String {
    format!(
"pub struct {name}ClientImpl(Client);"
    )
}

fn create_impl(name: &str) -> String {
    format!(
"impl {name}ClientImpl {{
    pub fn new(config: &SdkConfig) -> Self {{ Self(Client::new(config)) }}
}}")
}

fn create_trait(name: &str, methods: &[&str]) -> String {
    format!(
"pub trait {name}Client {{
{trait_methods}
}}", trait_methods = create_trait_methods(methods))
}

fn create_trait_impl(name: &str, methods: &[&str]) -> String {
    format!(
        "impl {name}Client for {name}ClientImpl {{
{trait_impl_methods}
}}", trait_impl_methods = create_trait_impl_methods(methods))
}

fn create_trait_borrow_impl(name: &str, methods: &[&str]) -> String {
    format!(
        "impl <T: {name}Client> {name}Client for &T {{
{borrow_impl_methods}
}}", borrow_impl_methods = create_trait_borrow_impl_methods(methods))
}

fn create_mock(name: &str, methods: &[&str]) -> String {
    format!(
        "#[cfg(feature = \"mockall\")]
mockall::mock! {{
    pub ed{name}Client {{}}
    impl {name}Client for ed{name}Client {{
{mock_methods}
    }}
}}", mock_methods = create_mock_methods(methods))
}

fn create_trait_methods(methods: &[&str]) -> String {
    methods.iter().map(|method| {
        let camel_method = snake_to_camel(method);
        format!(
"    fn {method}(&self, builder: {camel_method}InputBuilder) -> impl Future<Output = Result<{camel_method}Output, SdkError<{camel_method}Error>>>;"
        )
    }).collect::<Vec<_>>().join("\n")
}

fn create_trait_impl_methods(methods: &[&str]) -> String {
    methods.iter().map(|method| {
        let camel_method = snake_to_camel(method);
        format!(
"    fn {method}(&self, builder: {camel_method}InputBuilder) -> impl Future<Output = Result<{camel_method}Output, SdkError<{camel_method}Error>>> {{
        builder.send_with(&self.0)
    }}",
        )
    }).collect::<Vec<_>>().join("\n")
}

fn create_trait_borrow_impl_methods(methods: &[&str]) -> String {
    methods.iter().map(|method| {
        let camel_method = snake_to_camel(method);
        format!(
"    fn {method}(&self, builder: {camel_method}InputBuilder) -> impl Future<Output = Result<{camel_method}Output, SdkError<{camel_method}Error>>> {{
        (*self).{method}(builder)
    }}",
        )
    }).collect::<Vec<_>>().join("\n")
}

fn create_mock_methods(methods: &[&str]) -> String {
    methods.iter().map(|method| {
        let camel_method = snake_to_camel(method);
        format!(
"        async fn {method}(&self, builder: {camel_method}InputBuilder) -> Result<{camel_method}Output, SdkError<{camel_method}Error>>;"
        )
    }).collect::<Vec<_>>().join("\n")
}



fn snake_to_camel(s: &str) -> String {
    s.split('_').map(to_titlecase).collect::<Vec<_>>().join("")
}

fn camel_to_snake(s: &str) -> String {
    let (mut acc, curr) = s.chars().fold((String::new(), String::new()), |(mut acc, mut curr), c| {
        match curr.chars().last() {
            Some(x) if x.is_ascii_uppercase() => {
                if c.is_ascii_lowercase() {
                    let last = curr.pop();
                    acc.push_str(&curr);
                    if !acc.is_empty() && !acc.ends_with('_') { acc.push('_'); }
                    curr = String::new();
                    if let Some(l) = last { curr.push(l); }
                }
                curr.push(c);
            },
            Some(x) if x.is_ascii_lowercase() => {
                if c.is_ascii_uppercase() {
                    acc.push_str(&curr);
                    acc.push('_');
                    curr = String::new();
                }
                curr.push(c);
            }
            Some(_) => panic!(),
            None => curr.push(c),
        };
        (acc, curr)
    });
    acc.push_str(&curr);
    acc.to_ascii_lowercase()
}

fn to_titlecase(s: &str) -> String {
    s.chars().enumerate().map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("hello_world"), "HelloWorld");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(camel_to_snake("HelloWorld"), "hello_world");
    }

    #[test]
    fn test_camel_to_snake_2() {
        assert_eq!(camel_to_snake("XDHelloABCWorldXD"), "xd_hello_abc_world_xd");
    }
}