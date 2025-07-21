/**
 * Docker環境初期化プラグイン
 * アプリケーション起動時にDocker環境を初期化します
 */
import { useDockerStore } from '~/stores/dockerStore';

export default defineNuxtPlugin(async () => {
  // Docker環境の初期化
  const dockerStore = useDockerStore();

  // アプリケーション起動時にDocker環境を初期化
  try {
    await dockerStore.initializeDockerEnvironment();
    console.log('Docker環境の初期化が完了しました');
  } catch (error) {
    console.error('Docker環境の初期化中にエラーが発生しました:', error);
  }
});