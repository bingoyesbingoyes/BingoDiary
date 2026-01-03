<template>
    <div class="settings-overlay" @click.self="$emit('close')">
        <div class="settings-modal" :class="{ 'dark-mode': darkTheme }">
            <div class="settings-header">
                <h2>Settings</h2>
                <button @click="$emit('close')" class="close-btn">×</button>
            </div>

            <!-- Settings Tabs -->
            <div class="settings-tabs">
                <button
                    @click="currentTab = 'general'"
                    :class="{ active: currentTab === 'general' }"
                >General</button>
                <button
                    v-if="!isMobile"
                    @click="currentTab = 'shortcuts'"
                    :class="{ active: currentTab === 'shortcuts' }"
                >Shortcuts</button>
            </div>

            <div class="settings-content">
                <!-- General Tab -->
                <div v-if="currentTab === 'general'">
                    <!-- Storage Path (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>Storage Location</h3>
                        <div class="path-row">
                            <input type="text" :value="storagePath" readonly class="path-input" />
                            <button @click="$emit('change-path')" class="path-btn">📁</button>
                        </div>
                    </div>

                    <!-- Background Settings -->
                    <div class="settings-section">
                        <h3>Background</h3>
                        <div class="bg-type-tabs">
                            <button
                                v-for="type in ['preset', 'color', 'image']"
                                :key="type"
                                @click="$emit('update:bgType', type)"
                                :class="{ active: config.bgType === type }"
                            >
                                {{ type === 'preset' ? 'Presets' : type === 'color' ? 'Custom Color' : 'Image' }}
                            </button>
                        </div>

                        <!-- Preset Colors -->
                        <div v-if="config.bgType === 'preset'" class="preset-colors">
                            <div
                                v-for="preset in presetThemes"
                                :key="preset.name"
                                @click="$emit('update:bgValue', preset.name)"
                                class="preset-swatch"
                                :class="{ selected: config.bgValue === preset.name }"
                                :style="{ background: preset.light }"
                            >
                                <span class="preset-name">{{ preset.label }}</span>
                            </div>
                        </div>

                        <!-- Custom Color -->
                        <div v-if="config.bgType === 'color'" class="color-picker">
                            <input type="color" :value="config.bgValue" @input="$emit('update:bgValue', $event.target.value)" class="color-input" />
                            <span>{{ config.bgValue }}</span>
                        </div>

                        <!-- Image Upload -->
                        <div v-if="config.bgType === 'image'" class="image-upload">
                            <button @click="$emit('select-image')" class="upload-btn">Select Image</button>
                            <span v-if="config.bgValue" class="image-path">
                                {{ config.bgValue.split('/').pop() }}
                            </span>
                            <div class="fit-mode-selector" v-if="config.bgValue">
                                <label>Fit Mode:</label>
                                <select :value="config.bgFit" @change="$emit('update:bgFit', $event.target.value)">
                                    <option value="cover">Cover</option>
                                    <option value="contain">Contain</option>
                                    <option value="fill">Stretch</option>
                                    <option value="auto">Original</option>
                                </select>
                            </div>
                            <div class="opacity-slider" v-if="config.bgValue">
                                <label>Opacity: {{ Math.round((config.bgOpacity || 1) * 100) }}%</label>
                                <input
                                    type="range"
                                    :value="config.bgOpacity"
                                    @input="$emit('update:bgOpacity', parseFloat($event.target.value))"
                                    min="0.1"
                                    max="1"
                                    step="0.05"
                                />
                            </div>
                            <div class="opacity-slider" v-if="config.bgValue">
                                <label>Panel Transparency: {{ Math.round((config.panelOpacity || 0.45) * 100) }}%</label>
                                <input
                                    type="range"
                                    :value="config.panelOpacity"
                                    @input="$emit('update:panelOpacity', parseFloat($event.target.value))"
                                    min="0.1"
                                    max="1"
                                    step="0.05"
                                />
                            </div>
                        </div>
                    </div>

                    <!-- Window Settings (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>Window</h3>
                        <div class="window-size-row">
                            <div class="size-input-group">
                                <label>Width</label>
                                <input
                                    type="number"
                                    :value="config.windowWidth"
                                    @input="$emit('update:windowWidth', parseInt($event.target.value))"
                                    min="800"
                                    max="2560"
                                    class="size-input"
                                />
                            </div>
                            <span class="size-separator">×</span>
                            <div class="size-input-group">
                                <label>Height</label>
                                <input
                                    type="number"
                                    :value="config.windowHeight"
                                    @input="$emit('update:windowHeight', parseInt($event.target.value))"
                                    min="600"
                                    max="1440"
                                    class="size-input"
                                />
                            </div>
                            <button @click="$emit('apply-window-size')" class="apply-size-btn">Apply</button>
                        </div>
                    </div>

                    <!-- Password Settings (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>App Password</h3>
                        <div class="password-toggle">
                            <label>
                                <input type="checkbox" :checked="passwordEnabled" @change="$emit('toggle-password', $event.target.checked)" />
                                Enable startup password
                            </label>
                        </div>
                        <div v-if="passwordEnabled" class="password-inputs">
                            <input
                                type="password"
                                v-model="newPassword"
                                placeholder="New password"
                                class="pwd-input"
                            />
                            <input
                                type="password"
                                v-model="confirmPassword"
                                placeholder="Confirm password"
                                class="pwd-input"
                            />
                            <button @click="savePassword" class="save-pwd-btn">Set Password</button>
                        </div>
                    </div>
                </div>

                <!-- Shortcuts Tab -->
                <div v-if="currentTab === 'shortcuts'" class="shortcuts-tab">
                    <div class="shortcuts-group">
                        <h3>Navigation</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Go to Today</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>T</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Focus Search</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>F</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Open Settings</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>,</kbd></span>
                        </div>
                    </div>

                    <div class="shortcuts-group">
                        <h3>Editor</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Save</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>S</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Undo</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>Z</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Bold</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>B</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Italic</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>I</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Insert Link</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>K</kbd></span>
                        </div>
                    </div>

                    <div class="shortcuts-group">
                        <h3>View</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Toggle Preview</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>P</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Split Mode</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>⇧</kbd><kbd>P</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">Exit / Clear</span>
                            <span class="shortcut-keys"><kbd>Esc</kbd></span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="settings-footer" v-if="currentTab === 'general'">
                <button @click="$emit('save')" class="save-btn">Save</button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
    darkTheme: Boolean,
    isMobile: Boolean,
    storagePath: String,
    config: Object,
    presetThemes: Array,
    passwordEnabled: Boolean
});

const emit = defineEmits([
    'close',
    'save',
    'change-path',
    'select-image',
    'apply-window-size',
    'toggle-password',
    'save-password',
    'update:bgType',
    'update:bgValue',
    'update:bgFit',
    'update:bgOpacity',
    'update:panelOpacity',
    'update:windowWidth',
    'update:windowHeight'
]);

const currentTab = ref('general');
const newPassword = ref('');
const confirmPassword = ref('');

const savePassword = () => {
    if (newPassword.value && newPassword.value === confirmPassword.value) {
        emit('save-password', newPassword.value);
        newPassword.value = '';
        confirmPassword.value = '';
    }
};
</script>
