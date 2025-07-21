// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {
    enabled: true,
  },
  ssr: false,
  srcDir: 'src/',

  devServer: {
    port: 8765
  },

  /**
   * ビルド設定
   * Vuetifyをトランスパイル対象に追加し、ESモジュールをCommonJSに変換
   */
  build: {
    transpile: ['vuetify']
  },

  /**
   * TypeScript設定
   * 厳格な型チェックを有効化し、ビルド時の型チェックは無効化（パフォーマンス向上）
   */
  typescript: {
    strict: true,     // 厳格モード有効
    typeCheck: false  // ビルド時型チェック無効（別途実行）
  },

  /**
   * Nuxtモジュール設定
   * 多言語化、リンター、状態管理、アイコンのモジュールを組み込み
   */
  modules: [
    '@nuxtjs/i18n',    // 多言語化サポート
    '@nuxt/eslint',    // コード品質管理（ESLint）
    '@pinia/nuxt',     // 状態管理（Pinia）
    '@nuxt/icon'       // アイコン管理
  ],

  /**
   * Pinia設定
   * 状態管理の詳細設定
   */
  pinia: {
    storesDirs: ['./src/stores/**']
  },

  /**
   * 多言語化設定
   * 日本語をデフォルト言語として設定
   */
  i18n: {
    defaultLocale: 'ja',
    locales: [
      {
        code: 'ja',
        name: '日本語',
        file: 'ja.json'
      },
      {
        code: 'en',
        name: 'English',
        file: 'en.json'
      }
    ],
    lazy: true,
    langDir: 'i18n/locales/',
    strategy: 'no_prefix',
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    }
  },

  /**
   * ランタイム設定
   * 環境変数の公開設定
   */
  runtimeConfig: {
    // プライベート設定（サーバーサイドのみ）
    private: {},
    // パブリック設定（クライアントサイドでも利用可能）
    public: {
      appName: 'ProjectLens',
      appVersion: '0.1.0',
      isDevelopment: process.env.NODE_ENV === 'development'
    }
  },

  /**
   * Vite設定（Vuetify用カスタマイズ）
   * デバッグフラグの無効化とSSRでのVuetify外部化設定
   */
  vite: {
    // グローバル定数の定義
    define: {
      'process.env.DEBUG': false  // デバッグモード無効化
    },
    // SSR設定
    ssr: {
      noExternal: ['vuetify']       // VuetifyをSSRで外部化しない
    }
  },

  /**
   * CSSプリプロセッサー設定
   * VuetifyのメインスタイルとMaterial Design Iconsを読み込み
   */
  css: [
    'vuetify/lib/styles/main.sass',           // Vuetifyメインスタイル
    '@mdi/font/css/materialdesignicons.css'   // Material Designアイコン
  ],

  /**
   * アプリケーション設定
   * メタ情報とヘッド設定
   */
  app: {
    head: {
      title: 'ProjectLens',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'Multi-project dashboard with AI-powered task prioritization' },
        { name: 'format-detection', content: 'telephone=no' }
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ]
    }
  },




})