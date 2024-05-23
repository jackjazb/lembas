# lembas-frontend

This project contains a React Native frontend for lembas. It is configured around the Expo toolchain to be built and run on Android. To build and run the app, `expo` and `eas` need to be install globally with `npm`.

## Development

To start a development server, which allows the app to be opened in an emulator or physical device with USB debugging enabled, run `yarn start`. This requires the Android SDK, which can be installed using Android Studio, as well as the ADB debugging tool.

## Building

Expo builds using EAS. To run a local build which generates an installable APK, run `yarn build`.

## Testing

This project uses `jest` for unit tests. All tests can be run with `yarn test`.

## Design

This project using Material Design, as implemented by the React Native Paper library.

The font used by the icon is Adamina

### Entity Icons

Different app entities have icons which are used throughout to make them more recognisable. Currently, these are:
| Icon              | Entity                    |
|-------------------|---------------------------|
| Recipe            | 'notebook-outline'        |
| Ingredient        | 'package-variant-closed'  |
| User Ingredient   | 'account'                 |
