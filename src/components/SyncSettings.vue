<template>
    <div class="sync-settings">
        <!-- Google Client ID Setup -->
        <div class="settings-section" v-if="!clientId">
            <h3>{{ t('sync.googleCloudSetup') }}</h3>
            <p class="setup-info">
                {{ t('sync.setupInfo') }}
            </p>
            <ol class="setup-steps">
                <li>{{ t('sync.step1') }} <a href="#" @click.prevent="openExternal('https://console.cloud.google.com')">Google Cloud Console</a></li>
                <li>{{ t('sync.step2') }}</li>
                <li>{{ t('sync.step3') }}</li>
                <li>{{ t('sync.step4') }}</li>
                <li>{{ t('sync.step5') }}</li>
                <li>{{ t('sync.step6') }}</li>
            </ol>
            <div class="credentials-input">
                <input
                    type="text"
                    v-model="newClientId"
                    :placeholder="t('sync.clientId')"
                    class="client-input"
                />
                <input
                    type="password"
                    v-model="newClientSecret"
                    :placeholder="t('sync.clientSecretHint')"
                    class="client-input"
                />
                <button @click="saveCredentials" class="save-client-btn" :disabled="!newClientId">
                    {{ t('common.save') }}
                </button>
            </div>
        </div>

        <!-- Connection Success Message -->
        <div v-if="connectionSuccess" class="connection-success">
            <span class="success-icon">✓</span>
            <span>{{ t('sync.connectionSuccess') }}</span>
        </div>

        <!-- Account Connection -->
        <div class="settings-section" v-if="clientId">
            <h3>{{ t('sync.googleAccount') }}</h3>
            <div v-if="status.connected" class="account-connected">
                <div class="account-info">
                    <span class="account-icon">✓</span>
                    <span class="account-email">{{ status.accountEmail || t('sync.connected') }}</span>
                </div>
                <button @click="disconnect" class="disconnect-btn">{{ t('sync.disconnect') }}</button>
            </div>
            <div v-else class="account-disconnected">
                <p>{{ t('sync.connectHint') }}</p>
                <button @click="connect" class="connect-btn" :disabled="connecting">
                    {{ connecting ? t('sync.connecting') : t('sync.connectGoogleAccount') }}
                </button>
                <!-- Fallback when browser can't open -->
                <div v-if="authUrlFallback" class="auth-fallback">
                    <p class="fallback-hint">{{ t('sync.fallbackHint') }}</p>
                    <a :href="authUrlFallback" target="_blank" class="auth-link" @click="onAuthLinkClick">
                        {{ t('sync.clickToLogin') }}
                    </a>
                    <button @click="copyAuthUrl" class="copy-btn">{{ t('sync.copyLink') }}</button>
                </div>
                <!-- Mobile: manual code input -->
                <div v-if="showCodeInput" class="code-input-section">
                    <p class="code-hint" v-html="t('sync.codeHint')"></p>
                    <input
                        type="text"
                        v-model="manualCode"
                        :placeholder="t('sync.pasteCodePlaceholder')"
                        class="code-input"
                    />
                    <div class="code-buttons">
                        <button @click="submitManualCode" class="submit-code-btn" :disabled="!manualCode.trim() || connecting">
                            {{ connecting ? t('sync.verifying') : t('sync.submit') }}
                        </button>
                        <button @click="cancelCodeInput" class="cancel-btn">{{ t('common.cancel') }}</button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Sync Settings -->
        <div class="settings-section" v-if="clientId && status.connected">
            <h3>{{ t('sync.syncSettings') }}</h3>
            <div class="sync-option">
                <label>
                    <input
                        type="checkbox"
                        :checked="settings.enabled"
                        @change="updateSetting('enabled', $event.target.checked)"
                    />
                    {{ t('sync.enableAutoSync') }}
                </label>
            </div>
            <div class="sync-option" v-if="settings.enabled">
                <label>{{ t('sync.syncInterval') }}</label>
                <select
                    :value="settings.syncIntervalMinutes"
                    @change="updateSetting('syncIntervalMinutes', parseInt($event.target.value))"
                    class="interval-select"
                >
                    <option :value="5">{{ t('sync.every5min') }}</option>
                    <option :value="15">{{ t('sync.every15min') }}</option>
                    <option :value="30">{{ t('sync.every30min') }}</option>
                    <option :value="60">{{ t('sync.everyHour') }}</option>
                </select>
            </div>
        </div>

        <!-- Manual Sync -->
        <div class="settings-section" v-if="clientId && status.connected">
            <h3>{{ t('sync.manualSync') }}</h3>
            <div class="sync-status-row">
                <div class="last-sync" v-if="status.lastSync">
                    {{ t('sync.lastSync') }}: {{ formatLastSync(status.lastSync) }}
                </div>
                <div class="last-sync" v-else>
                    {{ t('sync.neverSynced') }}
                </div>
                <button
                    @click="syncNow"
                    class="sync-now-btn"
                    :disabled="status.syncing"
                >
                    {{ status.syncing ? t('sync.syncing') : t('sync.syncNow') }}
                </button>
            </div>
            <!-- Progress Bar -->
            <div v-if="syncProgress && status.syncing" class="sync-progress">
                <div class="progress-header">
                    <span class="progress-stage">{{ getStageLabel(syncProgress.stage) }}</span>
                    <span class="progress-count" v-if="syncProgress.total > 0">{{ syncProgress.current }} / {{ syncProgress.total }}</span>
                </div>
                <div class="progress-message">{{ syncProgress.message }}</div>
                <div class="progress-bar-container">
                    <div
                        class="progress-bar-fill"
                        :style="{ width: getProgressPercent(syncProgress) + '%' }"
                    ></div>
                </div>
            </div>
            <div v-if="syncError" class="sync-error">
                {{ syncError }}
            </div>
            <div v-if="syncResult" class="sync-result">
                <div v-if="syncResult.uploaded.length">
                    {{ t('sync.uploaded') }}: {{ syncResult.uploaded.length }} {{ t('sync.files') }}
                </div>
                <div v-if="syncResult.downloaded.length">
                    {{ t('sync.downloaded') }}: {{ syncResult.downloaded.length }} {{ t('sync.files') }}
                </div>
                <div v-if="!syncResult.uploaded.length && !syncResult.downloaded.length">
                    {{ t('sync.upToDate') }}
                </div>
            </div>
        </div>

        <!-- Change Client ID -->
        <div class="settings-section" v-if="clientId">
            <details class="advanced-settings">
                <summary>{{ t('sync.advancedSettings') }}</summary>
                <div class="advanced-content">
                    <p>{{ t('sync.currentClientId') }}: {{ clientId.substring(0, 20) }}...</p>
                    <button @click="resetClientId" class="reset-btn">
                        {{ t('sync.changeClientId') }}
                    </button>
                </div>
            </details>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { platform } from '@tauri-apps/plugin-os';
import { listen } from '@tauri-apps/api/event';

const props = defineProps({
    darkTheme: Boolean,
    t: { type: Function, required: true }
});

const emit = defineEmits(['sync-completed']);

const clientId = ref(null);
const newClientId = ref('');
const newClientSecret = ref('');
const status = ref({
    connected: false,
    syncing: false,
    lastSync: null,
    error: null,
    accountEmail: null
});
const settings = ref({
    enabled: false,
    syncMode: 'manual',
    syncIntervalMinutes: 15
});
const connecting = ref(false);
const syncError = ref(null);
const syncResult = ref(null);
const connectionSuccess = ref(false);
const syncProgress = ref(null);
const authUrlFallback = ref(null);
const showCodeInput = ref(false);
const manualCode = ref('');

let unlistenProgress = null;

onMounted(async () => {
    // Listen for sync progress events
    unlistenProgress = await listen('sync-progress', (event) => {
        syncProgress.value = event.payload;
    });
    await loadClientId();
    await loadStatus();
    await loadSettings();
});

onUnmounted(() => {
    if (unlistenProgress) {
        unlistenProgress();
    }
});

const loadClientId = async () => {
    try {
        clientId.value = await invoke('get_google_client_id');
    } catch (e) {
        console.error('Failed to load client ID:', e);
    }
};

const loadStatus = async () => {
    try {
        status.value = await invoke('get_sync_status');
    } catch (e) {
        console.error('Failed to load sync status:', e);
    }
};

const loadSettings = async () => {
    try {
        settings.value = await invoke('get_sync_settings');
    } catch (e) {
        console.error('Failed to load sync settings:', e);
    }
};

const saveClientId = async () => {
    if (!newClientId.value) return;
    try {
        await invoke('save_google_client_id', { clientId: newClientId.value });
        clientId.value = newClientId.value;
        newClientId.value = '';
    } catch (e) {
        console.error('Failed to save client ID:', e);
    }
};

const saveCredentials = async () => {
    if (!newClientId.value) return;
    try {
        await invoke('save_google_credentials', {
            clientId: newClientId.value,
            clientSecret: newClientSecret.value || null
        });
        clientId.value = newClientId.value;
        newClientId.value = '';
        newClientSecret.value = '';
    } catch (e) {
        console.error('Failed to save credentials:', e);
    }
};

const resetClientId = async () => {
    try {
        // Clear all sync credentials
        await invoke('clear_sync_credentials');
        clientId.value = null;
        status.value.connected = false;
        syncResult.value = null;
        syncError.value = null;
        authUrlFallback.value = null;
    } catch (e) {
        console.error('Failed to clear credentials:', e);
        // Fallback: just reset local state
        clientId.value = null;
    }
};

const connect = async () => {
    connecting.value = true;
    syncError.value = null;
    connectionSuccess.value = false;

    try {
        const currentPlatform = await platform();
        const isMobile = currentPlatform === 'android' || currentPlatform === 'ios';
        console.log('[Sync] Platform:', currentPlatform, 'isMobile:', isMobile);

        // Get auth URL
        const authUrl = await invoke('get_google_auth_url', { isMobile });
        console.log('[Sync] Auth URL obtained, length:', authUrl.length);

        if (isMobile) {
            // Mobile: open browser, user manually copies code
            console.log('[Sync] Mobile OAuth flow - manual code entry');
            // Always show code input UI for mobile
            showCodeInput.value = true;
            manualCode.value = '';
            try {
                await open(authUrl);
                console.log('[Sync] Browser opened');
            } catch (openErr) {
                console.error('[Sync] Failed to open browser:', openErr);
                // Also show fallback link if browser fails to open
                authUrlFallback.value = authUrl;
            }
            connecting.value = false;
        } else {
            // Desktop: use localhost callback server
            console.log('[Sync] Desktop OAuth flow - callback server');
            authUrlFallback.value = null;

            const callbackPromise = invoke('start_oauth_callback_server')
                .then(code => {
                    console.log('[Sync] Callback server received code');
                    return code;
                })
                .catch(err => {
                    console.error('[Sync] Callback server error:', err);
                    throw err;
                });

            await new Promise(resolve => setTimeout(resolve, 500));

            try {
                await open(authUrl);
            } catch (openErr) {
                console.error('[Sync] Failed to open browser:', openErr);
                authUrlFallback.value = authUrl;
                connecting.value = false;
                return;
            }

            const code = await callbackPromise;
            await invoke('handle_oauth_callback', { code, isMobile: false });
            await loadStatus();

            if (status.value.connected) {
                connectionSuccess.value = true;
                setTimeout(() => { connectionSuccess.value = false; }, 5000);
            }
        }
    } catch (e) {
        syncError.value = `Connection failed: ${e}`;
    } finally {
        connecting.value = false;
    }
};

// Poll for connection status (for mobile after OAuth redirect)
const pollConnectionStatus = async () => {
    let attempts = 0;
    const maxAttempts = 60; // Poll for up to 60 seconds

    const poll = async () => {
        attempts++;
        await loadStatus();

        if (status.value.connected) {
            connectionSuccess.value = true;
            connecting.value = false;
            setTimeout(() => { connectionSuccess.value = false; }, 5000);
            return;
        }

        if (attempts < maxAttempts) {
            setTimeout(poll, 1000);
        } else {
            connecting.value = false;
        }
    };

    poll();
};

const disconnect = async () => {
    try {
        await invoke('disconnect_google');
        await loadStatus();
        syncResult.value = null;
    } catch (e) {
        syncError.value = `Disconnect failed: ${e}`;
    }
};

const updateSetting = async (key, value) => {
    settings.value[key] = value;
    try {
        await invoke('save_sync_settings', { settings: settings.value });
    } catch (e) {
        console.error('Failed to save settings:', e);
    }
};

const syncNow = async () => {
    status.value.syncing = true;
    syncError.value = null;
    syncResult.value = null;
    syncProgress.value = null;

    try {
        const result = await invoke('start_sync');
        syncResult.value = result;
        await loadStatus();
        emit('sync-completed');
    } catch (e) {
        syncError.value = `Sync failed: ${e}`;
    } finally {
        status.value.syncing = false;
        syncProgress.value = null;
    }
};

const formatLastSync = (timestamp) => {
    if (!timestamp) return 'Never';
    const date = new Date(timestamp);
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / 60000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins} minutes ago`;

    const diffHours = Math.floor(diffMins / 60);
    if (diffHours < 24) return `${diffHours} hours ago`;

    return date.toLocaleDateString();
};

const openExternal = async (url) => {
    await open(url);
};

const copyAuthUrl = async () => {
    if (authUrlFallback.value) {
        try {
            await navigator.clipboard.writeText(authUrlFallback.value);
            alert(props.t('sync.linkCopied'));
        } catch (e) {
            // Fallback for older browsers
            const textArea = document.createElement('textarea');
            textArea.value = authUrlFallback.value;
            document.body.appendChild(textArea);
            textArea.select();
            document.execCommand('copy');
            document.body.removeChild(textArea);
            alert(props.t('sync.linkCopied'));
        }
    }
};

const onAuthLinkClick = () => {
    // Start polling when user clicks the link manually
    connecting.value = true;
    pollConnectionStatus();
};

const submitManualCode = async () => {
    if (!manualCode.value.trim()) return;

    connecting.value = true;
    syncError.value = null;

    try {
        // Extract code from URL if user pasted full URL
        let code = manualCode.value.trim();
        const codeMatch = code.match(/[?&]code=([^&]+)/);
        if (codeMatch) {
            code = decodeURIComponent(codeMatch[1]);
        }

        console.log('[Sync] Submitting manual code...');
        await invoke('handle_oauth_callback', { code, isMobile: true });
        await loadStatus();

        if (status.value.connected) {
            connectionSuccess.value = true;
            showCodeInput.value = false;
            manualCode.value = '';
            setTimeout(() => { connectionSuccess.value = false; }, 5000);
        }
    } catch (e) {
        syncError.value = `${props.t('sync.authFailed')}: ${e}`;
    } finally {
        connecting.value = false;
    }
};

const cancelCodeInput = () => {
    showCodeInput.value = false;
    manualCode.value = '';
};

const getStageLabel = (stage) => {
    const labels = {
        'init': props.t('sync.stageConnecting'),
        'entries': props.t('sync.stageDiaryEntries'),
        'tags': props.t('sync.stageTags'),
        'images': props.t('sync.stageImages')
    };
    return labels[stage] || stage;
};

const getProgressPercent = (progress) => {
    if (!progress || progress.total === 0) return 0;
    return Math.round((progress.current / progress.total) * 100);
};
</script>

<style scoped>
.sync-settings {
    padding: 0;
}

.settings-section {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid rgba(128, 128, 128, 0.2);
}

.settings-section:last-child {
    border-bottom: none;
    margin-bottom: 0;
}

.settings-section h3 {
    margin: 0 0 0.75rem;
    font-size: 0.9rem;
    font-weight: 600;
    color: inherit;
}

.setup-info {
    font-size: 0.85rem;
    margin-bottom: 0.75rem;
    opacity: 0.8;
}

.setup-steps {
    font-size: 0.8rem;
    padding-left: 1.25rem;
    margin-bottom: 1rem;
    line-height: 1.6;
}

.setup-steps a {
    color: #4a90d9;
    text-decoration: none;
}

.setup-steps a:hover {
    text-decoration: underline;
}

.client-id-input {
    display: flex;
    gap: 0.5rem;
}

.credentials-input {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.credentials-input .client-input {
    width: 100%;
    box-sizing: border-box;
}

.client-input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid rgba(128, 128, 128, 0.3);
    border-radius: 4px;
    font-size: 0.85rem;
    background: rgba(255, 255, 255, 0.1);
}

.save-client-btn,
.connect-btn,
.disconnect-btn,
.sync-now-btn,
.reset-btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
    transition: opacity 0.2s;
}

.save-client-btn:disabled,
.connect-btn:disabled,
.sync-now-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.save-client-btn,
.connect-btn {
    background: #4285f4;
    color: white;
}

.save-client-btn:hover:not(:disabled),
.connect-btn:hover:not(:disabled) {
    background: #3367d6;
}

.disconnect-btn,
.reset-btn {
    background: rgba(128, 128, 128, 0.2);
    color: inherit;
}

.disconnect-btn:hover,
.reset-btn:hover {
    background: rgba(128, 128, 128, 0.3);
}

.sync-now-btn {
    background: #34a853;
    color: white;
}

.sync-now-btn:hover:not(:disabled) {
    background: #2d8e47;
}

.account-connected {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
}

.account-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.account-icon {
    color: #34a853;
    font-weight: bold;
}

.account-email {
    font-size: 0.9rem;
}

.account-disconnected p {
    font-size: 0.85rem;
    margin-bottom: 0.75rem;
    opacity: 0.8;
}

.auth-fallback {
    margin-top: 1rem;
    padding: 0.75rem;
    background: rgba(255, 193, 7, 0.15);
    border: 1px solid rgba(255, 193, 7, 0.4);
    border-radius: 6px;
}

.fallback-hint {
    font-size: 0.8rem;
    margin-bottom: 0.5rem;
    color: #b38600;
}

.auth-link {
    display: block;
    padding: 0.5rem 0.75rem;
    background: #4285f4;
    color: white;
    text-decoration: none;
    border-radius: 4px;
    text-align: center;
    font-size: 0.85rem;
    margin-bottom: 0.5rem;
}

.auth-link:active {
    background: #3367d6;
}

.copy-btn {
    width: 100%;
    padding: 0.4rem 0.75rem;
    background: rgba(128, 128, 128, 0.2);
    border: none;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
}

.code-input-section {
    margin-top: 1rem;
    padding: 0.75rem;
    background: rgba(66, 133, 244, 0.1);
    border: 1px solid rgba(66, 133, 244, 0.3);
    border-radius: 6px;
}

.code-hint {
    font-size: 0.8rem;
    margin-bottom: 0.75rem;
    line-height: 1.5;
    opacity: 0.9;
}

.code-input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid rgba(128, 128, 128, 0.3);
    border-radius: 4px;
    font-size: 0.85rem;
    background: rgba(255, 255, 255, 0.1);
    box-sizing: border-box;
    margin-bottom: 0.5rem;
}

.code-buttons {
    display: flex;
    gap: 0.5rem;
}

.submit-code-btn {
    flex: 1;
    padding: 0.5rem;
    background: #4285f4;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 0.85rem;
    cursor: pointer;
}

.submit-code-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.cancel-btn {
    padding: 0.5rem 1rem;
    background: rgba(128, 128, 128, 0.2);
    border: none;
    border-radius: 4px;
    font-size: 0.85rem;
    cursor: pointer;
}

.sync-option {
    margin-bottom: 0.75rem;
}

.sync-option label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
}

.interval-select {
    padding: 0.35rem 0.5rem;
    border: 1px solid rgba(128, 128, 128, 0.3);
    border-radius: 4px;
    font-size: 0.85rem;
    background: rgba(255, 255, 255, 0.1);
    margin-left: 0.5rem;
}

.sync-status-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
}

.last-sync {
    font-size: 0.85rem;
    opacity: 0.7;
}

.sync-error {
    margin-top: 0.75rem;
    padding: 0.5rem;
    background: rgba(234, 67, 53, 0.1);
    border: 1px solid rgba(234, 67, 53, 0.3);
    border-radius: 4px;
    color: #ea4335;
    font-size: 0.85rem;
}

.connection-success {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    background: rgba(52, 168, 83, 0.15);
    border: 1px solid rgba(52, 168, 83, 0.4);
    border-radius: 8px;
    color: #34a853;
    font-size: 0.9rem;
    font-weight: 500;
    animation: slideIn 0.3s ease-out;
}

.connection-success .success-icon {
    font-size: 1.1rem;
    font-weight: bold;
}

@keyframes slideIn {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.sync-result {
    margin-top: 0.75rem;
    padding: 0.5rem;
    background: rgba(52, 168, 83, 0.1);
    border: 1px solid rgba(52, 168, 83, 0.3);
    border-radius: 4px;
    color: #34a853;
    font-size: 0.85rem;
}

.sync-progress {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: rgba(66, 133, 244, 0.1);
    border: 1px solid rgba(66, 133, 244, 0.3);
    border-radius: 4px;
}

.progress-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.35rem;
}

.progress-stage {
    font-size: 0.8rem;
    font-weight: 600;
    color: #4285f4;
}

.progress-count {
    font-size: 0.75rem;
    opacity: 0.8;
}

.progress-message {
    font-size: 0.8rem;
    margin-bottom: 0.5rem;
    opacity: 0.9;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.progress-bar-container {
    height: 6px;
    background: rgba(128, 128, 128, 0.2);
    border-radius: 3px;
    overflow: hidden;
}

.progress-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #4285f4, #34a853);
    border-radius: 3px;
    transition: width 0.2s ease;
}

.advanced-settings {
    font-size: 0.85rem;
}

.advanced-settings summary {
    cursor: pointer;
    opacity: 0.7;
}

.advanced-settings summary:hover {
    opacity: 1;
}

.advanced-content {
    margin-top: 0.75rem;
    padding: 0.75rem;
    background: rgba(128, 128, 128, 0.1);
    border-radius: 4px;
}

.advanced-content p {
    margin-bottom: 0.5rem;
    font-family: monospace;
    font-size: 0.8rem;
    opacity: 0.7;
}
</style>
