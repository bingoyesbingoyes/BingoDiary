<template>
    <div class="mobile-topbar" :class="{ 'dark-mode': darkTheme }">
        <div class="mobile-topbar-left">
            <button class="mobile-menu-btn" @click="$emit('open-drawer')">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                    <line x1="4" y1="7" x2="20" y2="7"/>
                    <line x1="4" y1="12" x2="20" y2="12"/>
                    <line x1="4" y1="17" x2="20" y2="17"/>
                </svg>
            </button>
        </div>
        <div class="mobile-topbar-center" @click="$emit('open-calendar')">
            <span class="mobile-date-main">{{ dateDisplay }}</span>
            <span class="mobile-date-year">{{ year }}</span>
        </div>
        <div class="mobile-topbar-right">
            <!-- Auto-save status indicator -->
            <div class="save-status" :class="{ saving: isSaving, saved: justSaved }">
                <svg v-if="isSaving" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="saving-icon">
                    <circle cx="12" cy="12" r="10"/>
                </svg>
                <svg v-else-if="justSaved" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="20 6 9 17 4 12"/>
                </svg>
            </div>
            <!-- Sync button (rightmost to align with editor buttons) -->
            <button class="mobile-sync-btn" :class="{ connected: syncConnected, syncing: syncSyncing }" @click="$emit('open-sync')">
                <svg width="20" height="20" viewBox="0 0 24 24" :class="{ spinning: syncSyncing }">
                    <path d="M4 16.5A4.5 4.5 0 0 1 5.5 8a5.5 5.5 0 0 1 10.3-1.5 4.5 4.5 0 0 1 4.7 6.3A3.5 3.5 0 0 1 18 19H6.5A4.5 4.5 0 0 1 4 16.5z"
                        :fill="syncConnected ? 'currentColor' : 'none'"
                        :fill-opacity="syncConnected ? 0.15 : 0"
                        stroke="currentColor"
                        stroke-width="1.5"
                        stroke-linecap="round"
                        stroke-linejoin="round"/>
                </svg>
            </button>
        </div>
    </div>
</template>

<script setup>
defineProps({
    darkTheme: Boolean,
    dateDisplay: String,
    year: [String, Number],
    isSaving: Boolean,
    justSaved: Boolean,
    syncConnected: Boolean,
    syncSyncing: Boolean
});

defineEmits(['open-drawer', 'open-calendar', 'open-sync']);
</script>
