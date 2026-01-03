<template>
    <button class="sync-status" :class="statusClass" @click.stop="handleClick" :title="statusTitle" type="button">
        <span class="sync-icon" :class="{ spinning: syncing }">
            <!-- Simple fluffy cloud -->
            <svg width="20" height="20" viewBox="0 0 24 24">
                <path d="M4 16.5A4.5 4.5 0 0 1 5.5 8a5.5 5.5 0 0 1 10.3-1.5 4.5 4.5 0 0 1 4.7 6.3A3.5 3.5 0 0 1 18 19H6.5A4.5 4.5 0 0 1 4 16.5z"
                    :fill="connected ? 'currentColor' : 'none'"
                    :fill-opacity="connected ? 0.15 : 0"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"/>
            </svg>
        </span>
        <span v-if="showLabel" class="sync-label">{{ statusLabel }}</span>
    </button>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
    connected: {
        type: Boolean,
        default: false
    },
    syncing: {
        type: Boolean,
        default: false
    },
    lastSync: {
        type: String,
        default: null
    },
    error: {
        type: String,
        default: null
    },
    showLabel: {
        type: Boolean,
        default: false
    },
    onClick: {
        type: Function,
        default: null
    }
});

const handleClick = (e) => {
    e.stopPropagation();
    if (props.onClick) {
        props.onClick();
    }
};

const statusClass = computed(() => {
    if (props.syncing) return 'syncing';
    if (props.error) return 'error';
    if (props.connected) return 'connected';
    return 'disconnected';
});

const statusLabel = computed(() => {
    if (props.syncing) return 'Syncing...';
    if (props.error) return 'Sync Error';
    if (props.connected) return 'Synced';
    return 'Not Connected';
});

const statusTitle = computed(() => {
    if (props.syncing) return 'Synchronizing with Google Drive...';
    if (props.error) return `Sync error: ${props.error}`;
    if (props.connected && props.lastSync) {
        const date = new Date(props.lastSync);
        return `Last synced: ${date.toLocaleString()}`;
    }
    if (props.connected) return 'Connected to Google Drive';
    return 'Click to set up Google Drive sync';
});
</script>

<style scoped>
.sync-status {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.8rem;
    transition: background 0.2s, opacity 0.2s;
    opacity: 0.7;
    -webkit-app-region: no-drag;
    pointer-events: auto;
    user-select: none;
    /* Button reset */
    border: none;
    background: transparent;
    font-family: inherit;
    outline: none;
}

.sync-status:hover {
    opacity: 1;
    background: rgba(128, 128, 128, 0.1);
}

.sync-icon {
    display: flex;
    align-items: center;
    justify-content: center;
}

.sync-icon svg {
    display: block;
}

.sync-status.connected .sync-icon {
    color: #34a853;
}

.sync-status.syncing .sync-icon {
    color: #4285f4;
}

.sync-status.error .sync-icon {
    color: #ea4335;
}

.sync-status.disconnected .sync-icon {
    opacity: 0.5;
}

.spinning {
    animation: spin 1s linear infinite;
}

@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

.sync-label {
    font-size: 0.75rem;
}
</style>
