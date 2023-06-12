# Frontend Architecture

## Overview

The following is a brief overview of the frontend architecture.  
This document applies to both the studentpage and adminpage frontends.  
For more information on the frontend frameworks used, see the [Frameworks](/Doc/Frontend/frameworks.md) document.

## Project Layout Overview

### `src/`

The `src/` directory contains all of the source code for the frontend.  
Since frontend compiles to WebAssembly, there exists an entry point for the application - located in `src/main.rs`.  
The first rendered component of the frontend that is the `App` component, which is defined in `src/lib.rs`.

### `src/api/`

The `src/api/` directory contains all of the code related to the API calls.  
The client side API calls are defined in `src/api/mod.rs`.  
See the [API](/Doc/API/APIDOC.md) document for more information on each endpoint.

### `src/assets/`

The `src/assets/` directory contains all of the static assets for the frontend.  
This currently only includes `src/assets/main.css`, which is the main stylesheet for the frontend.  
The stylesheet is very minimal, and only contains a few custom styles not achievable with bootstrap.  
The assets directory should be expanded to include any images, icons, fonts, or other static assets.

### `src/components/`

The `src/components/` directory contains all of the Yew components used in the frontend.  
Each component is defined in its own file and uses Bootstrap for styling and functionality.  
See the [Code Docs](https://www.tftinker.tech/EWU-CSCD488-490-Senior-Project/Doc/code/) for more information on each component.  
See the [Frameworks](/Doc/Frontend/frameworks.md) document for more information on how the components are created.

### `src/pages/`

The `src/pages/` directory contains all of the pages used in the frontend.  
Since the frontend is a single page application, each page is defined as a component.  
Each page is defined in its own file and uses Bootstrap for styling and functionality.  
See the [Code Docs](https://www.tftinker.tech/EWU-CSCD488-490-Senior-Project/Doc/code/) for more information on each page.

### `src/stores/`

The `src/stores/` directory contains all of the yewdux stores used in the frontend.  
Each store represents the data stored within local storage.  
Local storage is mostly used within the frontend to store the form data between pages.

## Routes & Pages

Since this is a single page application, the frontend uses a router to handle routing.  
The frontend uses the [yew_router](https://docs.rs/yew-router/0.17.0/yew_router/) crate to handle routing.  
All routes are defined within the `src/lib.rs` file.  
Each route maps a path to a page component.