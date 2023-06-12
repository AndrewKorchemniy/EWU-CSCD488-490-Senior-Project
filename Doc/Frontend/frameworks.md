# Frameworks & Dependencies

The following is a brief overview of the frontend frameworks and dependencies used.

## Bootstrap

_If you're not using a CSS framework, you're probably doing it wrong._  
The frontend is using the latest version of Bootstrap (5.3) for styling and functionality.  
Within the [code documentation](https://www.tftinker.tech/EWU-CSCD488-490-Senior-Project/Doc/code/), the purpose and use of any Bootstrap components are noted.  
Bootstrap [utility classes](https://getbootstrap.com/docs/5.3/layout/utilities/) are also used throughout the frontend.  
See the [Bootstrap Docs](https://getbootstrap.com/docs/5.3/getting-started/introduction/) for more information.

All custom styles that cannot be achieved within Bootstrap are defined in `src/assets/main.css`.

## Yew

The frontend is written in [Yew](https://yew.rs/docs/getting-started/introduction), a Rust framework for building web apps.  
Yew is a component based framework, similar to React, Vue, and Angular.  
There are two types of components in Yew: functional components and struct components.  
The frontend uses functional components, which are defined as functions that return a `Html` type using the `html!` macro.  
You will find many examples online using both. However, functional components are the preferred method of writing components in Yew.

## Dependencies

- [Yew](https://yew.rs/docs/getting-started/introduction)  
  A modern Rust framework for creating multithreaded front-end web apps using WebAssembly

- [Bootstrap](https://getbootstrap.com/docs/5.3/getting-started/introduction/)  
  The world’s most popular front-end open source toolkit, featuring a responsive grid system, extensive prebuilt components, and powerful JavaScript plugins.

- [Yewdux](https://docs.rs/yewdux/latest/yewdux/)  
  Simple state management for Yew applications.  
  See the [book](https://intendednull.github.io/yewdux/) for more details.

- [Yew Router](https://docs.rs/yew-router/0.17.0/yew_router/)  
  Provides routing faculties using the browser history API to build Single Page Applications (SPAs) using Yew web framework.

- [Stylist](https://docs.rs/stylist/latest/stylist/)  
  Stylist is a CSS-in-Rust styling solution for WebAssembly Applications.  
  It provides a convenient way to scope your styles to a component.

- [Gloo](https://docs.rs/gloo/latest/gloo/)  
  Gloo is a modular toolkit for building fast and reliable libraries and apps with Rust and WebAssembly.  
  This provides a way to interact with the browser's Web APIs from within Rust, such as logging to the console.

- [Yew-OAuth2](https://docs.rs/yew-oauth2/latest/yew_oauth2/)  
  Yew components to implement OAuth2 and OpenID Connect logins.

- [serde](https://docs.rs/serde/latest/serde/)  
  Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

- [serde_json](https://docs.rs/serde_json/latest/serde_json/)  
  Serde JSON is a serde-based JSON library for Rust.

- [wasm-bindgen-futures](https://docs.rs/wasm-bindgen-futures/latest/wasm_bindgen_futures/)  
  This crate provides a bridge for working with JavaScript Promise types as a Rust Future, and similarly contains utilities to turn a rust Future into a JavaScript Promise. This can be useful when working with asynchronous or otherwise blocking work in Rust (wasm), and provides the ability to interoperate with JavaScript events and JavaScript I/O primitives.

- [reqwasm](https://docs.rs/reqwasm/latest/reqwasm/)  
  HTTP requests library for WASM apps. It provides idiomatic Rust bindings for the web_sys fetch and WebSocket API.

- [chrono](https://docs.rs/chrono/latest/chrono/)  
  A crate for dealing with dates and times in Rust.  
  Compatible with serde and serde_json for serialization and deserialization.
