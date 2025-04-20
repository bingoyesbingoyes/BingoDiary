const { configure } = require("quasar/wrappers");

module.exports = configure(function (/* ctx */) {
    return {
        boot: ["global"],

        css: ["app.scss"],

        extras: ["material-icons"],
        // extras: ["roboto-font", "material-icons"],

        build: {
            target: {
                browser: [
                    "es2019",
                    "edge88",
                    "firefox78",
                    "chrome87",
                    "safari13.1",
                ],
                node: "node20",
            },

            vueRouterMode: "hash", // available values: 'hash', 'history'
        },

        devServer: {
            open: true,
        },

        framework: {
            config: {
                // dark: true
            },

            plugins: [],
        },

        animations: [],

        ssr: {
            pwa: false,

            prodPort: 3000, // The default port that the production server should use

            middlewares: [
                "render", // keep this as last one
            ],
        },

        pwa: {
            workboxMode: "generateSW", // or 'injectManifest'
            injectPwaMetaTags: true,
            swFilename: "sw.js",
            manifestFilename: "manifest.json",
            useCredentialsForManifestTag: false,
        },

        cordova: {},

        capacitor: {
            hideSplashscreen: true,
        },

        electron: {
            inspectPort: 5858,

            bundler: "builder", // 'packager' or 'builder'

            packager: {},

            builder: {
                appId: "BingoDiary",
                productName: "BingoDiary",
                icon: "src-electron/icons/icon.ico",
                win: {
                    target: [
                        {
                            target: "nsis",
                            arch: ["x64", "ia32"],
                        },
                    ],
                    icon: "src-electron/icons/icon.ico",
                },
                nsis: {
                    oneClick: false,
                    allowToChangeInstallationDirectory: true,
                    installerIcon: "src-electron/icons/icon.ico",
                    uninstallerIcon: "src-electron/icons/icon.ico",
                    installerHeaderIcon: "src-electron/icons/icon.ico",
                    createDesktopShortcut: true,
                    createStartMenuShortcut: true,
                    shortcutName: "BingoDiary",
                },
            },
        },

        bex: {
            contentScripts: ["my-content-script"],
        },
    };
});
