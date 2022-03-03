# gun-gui

First, clone the lastest version of gun into `src-tauri/src/`:
```
git clone https://github.com/GoUpNumber/gun && cp -r gun/src/* src-tauri/src/ 
```

Replace the `Cargo.toml` with the one that includes tauri. (should be fixed by some install script that appends tauri dependencies to gun's cargo.toml)
```
mv Cargo.toml src-tauri/
```

`src-tauri/src/main.rs` contains the bindings for Tauri, which are called from the components in `src/components/`.


## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn tauri:serve
```

### Compiles and minifies for production
```
yarn tauri:build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
