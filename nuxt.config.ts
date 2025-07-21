// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {
    enabled: true,
  },
  ssr: false,
  srcDir: 'src/',

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
   * 多言語化、リンターのモジュールを組み込み
   */
  modules: [
    '@nuxtjs/i18n',    // 多言語化サポート
    '@nuxt/eslint'     // コード品質管理（ESLint）
  ],

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




})