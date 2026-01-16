<template>
    <!-- Lock Screen -->
    <LockScreen
        v-if="isLocked"
        :dark-theme="darkTheme"
        :error="lockError"
        :t="t"
        @unlock="handleUnlock"
    />

    <!-- Settings Modal -->
    <div v-if="showSettings" class="settings-overlay" @click.self="closeSettings">
        <div class="settings-modal" :class="{ 'dark-mode': darkTheme }">
            <div class="settings-header">
                <h2>{{ t('settings.title') }}</h2>
                <button @click="closeSettings" class="close-btn">×</button>
            </div>

            <!-- Settings Tabs -->
            <div class="settings-tabs">
                <button
                    @click="settingsTab = 'general'"
                    :class="{ active: settingsTab === 'general' }"
                >{{ t('settings.tabs.general') }}</button>
                <button
                    @click="settingsTab = 'sync'"
                    :class="{ active: settingsTab === 'sync' }"
                >{{ t('settings.tabs.sync') }}</button>
                <button
                    v-if="!isMobile"
                    @click="settingsTab = 'shortcuts'"
                    :class="{ active: settingsTab === 'shortcuts' }"
                >{{ t('settings.tabs.shortcuts') }}</button>
            </div>

            <div class="settings-content">
                <!-- General Tab -->
                <div v-if="settingsTab === 'general'">
                    <!-- Storage Path (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>{{ t('settings.storageLocation') }}</h3>
                        <div class="path-row">
                            <input type="text" :value="storagePath" readonly class="path-input" />
                            <button @click="changeStoragePath" class="path-btn">📁</button>
                        </div>
                    </div>

                    <!-- Background Settings -->
                    <div class="settings-section">
                        <h3>{{ t('settings.background') }}</h3>
                        <div class="bg-type-tabs">
                            <button
                                v-for="type in ['preset', 'color', 'image']"
                                :key="type"
                                @click="switchBgType(type)"
                                :class="{ active: config.bgType === type }"
                            >
                                {{ type === 'preset' ? t('settings.bg.presets') : type === 'color' ? t('settings.bg.customColor') : t('settings.bg.image') }}
                            </button>
                        </div>

                        <!-- Preset Colors -->
                        <div v-if="config.bgType === 'preset'" class="preset-colors">
                            <div
                                v-for="preset in presetThemes"
                                :key="preset.name"
                                @click="config.bgValue = preset.name"
                                class="preset-swatch"
                                :class="{ selected: config.bgValue === preset.name }"
                                :style="{ background: preset.light }"
                            >
                                <span class="preset-name">{{ preset.label }}</span>
                            </div>
                        </div>

                        <!-- Custom Color -->
                        <div v-if="config.bgType === 'color'" class="color-picker">
                            <input type="color" v-model="config.bgValue" class="color-input" />
                            <span>{{ config.bgValue }}</span>
                        </div>

                        <!-- Image Upload -->
                        <div v-if="config.bgType === 'image'" class="image-upload">
                            <button @click="selectBgImage" class="upload-btn">{{ t('settings.selectImage') }}</button>
                            <span v-if="config.bgValue && config.bgType === 'image'" class="image-path">
                                {{ config.bgValue.split('/').pop() }}
                            </span>
                            <div class="fit-mode-selector" v-if="config.bgValue">
                                <label>{{ t('settings.fitMode') }}</label>
                                <select v-model="config.bgFit">
                                    <option value="cover">{{ t('settings.fitMode.cover') }}</option>
                                    <option value="contain">{{ t('settings.fitMode.contain') }}</option>
                                    <option value="fill">{{ t('settings.fitMode.stretch') }}</option>
                                    <option value="auto">{{ t('settings.fitMode.original') }}</option>
                                </select>
                            </div>
                            <div class="opacity-slider" v-if="config.bgValue">
                                <label>{{ t('settings.opacity') }} {{ Math.round((config.bgOpacity || 1) * 100) }}%</label>
                                <input
                                    type="range"
                                    v-model.number="config.bgOpacity"
                                    min="0.1"
                                    max="1"
                                    step="0.05"
                                />
                            </div>
                            <div class="opacity-slider" v-if="config.bgValue">
                                <label>{{ t('settings.panelTransparency') }} {{ Math.round((config.panelOpacity || 0.45) * 100) }}%</label>
                                <input
                                    type="range"
                                    v-model.number="config.panelOpacity"
                                    min="0.1"
                                    max="1"
                                    step="0.05"
                                />
                            </div>
                        </div>
                    </div>

                    <!-- Language Settings -->
                    <div class="settings-section">
                        <h3>{{ t('settings.language') }}</h3>
                        <select v-model="config.language" class="settings-select" @change="onLanguageChange">
                            <option v-for="lang in availableLanguages" :key="lang.code" :value="lang.code">
                                {{ lang.name }}
                            </option>
                        </select>
                    </div>

                    <!-- Window Settings (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>{{ t('settings.window') }}</h3>
                        <div class="window-size-row">
                            <div class="size-input-group">
                                <label>{{ t('settings.width') }}</label>
                                <input
                                    type="number"
                                    v-model.number="config.windowWidth"
                                    min="800"
                                    max="2560"
                                    class="size-input"
                                />
                            </div>
                            <span class="size-separator">×</span>
                            <div class="size-input-group">
                                <label>{{ t('settings.height') }}</label>
                                <input
                                    type="number"
                                    v-model.number="config.windowHeight"
                                    min="600"
                                    max="1440"
                                    class="size-input"
                                />
                            </div>
                            <button @click="applyWindowSize" class="apply-size-btn">{{ t('settings.apply') }}</button>
                        </div>
                    </div>

                    <!-- Password Settings (Desktop only) -->
                    <div v-if="!isMobile" class="settings-section">
                        <h3>{{ t('settings.appPassword') }}</h3>
                        <div class="password-toggle">
                            <label>
                                <input type="checkbox" v-model="passwordEnabled" @change="togglePassword" />
                                {{ t('settings.enablePassword') }}
                            </label>
                        </div>
                        <div v-if="passwordEnabled" class="password-inputs">
                            <input
                                type="password"
                                v-model="newPassword"
                                :placeholder="t('settings.newPassword')"
                                class="pwd-input"
                            />
                            <input
                                type="password"
                                v-model="confirmPassword"
                                :placeholder="t('settings.confirmPassword')"
                                class="pwd-input"
                            />
                            <button @click="savePassword" class="save-pwd-btn">{{ t('settings.setPassword') }}</button>
                        </div>
                    </div>
                </div>

                <!-- Shortcuts Tab -->
                <div v-if="settingsTab === 'shortcuts'" class="shortcuts-tab">
                    <div class="shortcuts-group">
                        <h3>{{ t('shortcuts.navigation') }}</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.goToToday') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>T</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.focusSearch') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>F</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.openSettings') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>,</kbd></span>
                        </div>
                    </div>

                    <div class="shortcuts-group">
                        <h3>{{ t('shortcuts.editor') }}</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.save') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>S</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.undo') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>Z</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.bold') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>B</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.italic') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>I</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.insertLink') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>K</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.cutLine') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>X</kbd></span>
                        </div>
                    </div>

                    <div class="shortcuts-group">
                        <h3>{{ t('shortcuts.view') }}</h3>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.togglePreview') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>P</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.splitMode') }}</span>
                            <span class="shortcut-keys"><kbd>⌘</kbd><kbd>⇧</kbd><kbd>P</kbd></span>
                        </div>
                        <div class="shortcut-item">
                            <span class="shortcut-desc">{{ t('shortcuts.exitClear') }}</span>
                            <span class="shortcut-keys"><kbd>Esc</kbd></span>
                        </div>
                    </div>
                </div>

                <!-- Sync Tab -->
                <div v-if="settingsTab === 'sync'" class="sync-tab">
                    <SyncSettings
                        :dark-theme="darkTheme"
                        :t="t"
                        @sync-completed="handleSyncCompleted"
                    />
                </div>
            </div>

            <div class="settings-footer" v-if="settingsTab === 'general'">
                <button @click="saveSettings" class="save-btn">{{ t('settings.save') }}</button>
            </div>
        </div>
    </div>

    <!-- Main App -->
    <div v-show="!isLocked" class="main-wrapper" :class="{ 'dark-mode': darkTheme, 'mobile': isMobile }" :style="bgBaseStyle">
        <!-- Background Image Overlay (for opacity support) -->
        <div v-if="config.bgType === 'image' && config.bgValue" class="bg-image-overlay" :style="bgImageStyle"></div>
        <!-- Mobile Top Bar -->
        <MobileTopBar
            v-if="isMobile"
            :dark-theme="darkTheme"
            :date-display="currentDateDisplay"
            :year="currentYear"
            :is-saving="isSaving"
            :just-saved="justSaved"
            :sync-connected="syncStatus.connected"
            :sync-syncing="syncStatus.syncing"
            @open-drawer="drawerOpen = true"
            @open-calendar="mobileView = 'calendar'; drawerOpen = true"
            @open-sync="handleSyncClick"
        />

        <!-- Mobile Drawer Overlay -->
        <div v-if="isMobile && drawerOpen" class="drawer-overlay" @click="drawerOpen = false"></div>

        <!-- Custom Titlebar (Desktop only) -->
        <div v-if="!isMobile" class="custom-titlebar" :class="{ 'dark-mode': darkTheme }" @mousedown="startDrag">
            <div class="titlebar-drag-region"></div>
            <div class="window-controls">
                <button class="control-btn close" @click="closeWindow" title="Close">
                    <span></span>
                </button>
                <button class="control-btn minimize" @click="minimizeWindow" title="Minimize">
                    <span></span>
                </button>
                <button class="control-btn maximize" @click="toggleMaximize" title="Maximize">
                    <span></span>
                </button>
            </div>
            <div class="titlebar-title">Bingo Diary</div>
            <div class="titlebar-actions">
                <!-- Sync Button: Quick sync when connected, open settings when not -->
                <button
                    class="settings-btn sync-btn"
                    :class="{ 'syncing': syncStatus.syncing, 'connected': syncStatus.connected, 'just-synced': justSynced, 'sync-failed': syncFailed }"
                    @click="handleSyncClick"
                    :title="syncStatus.connected ? (syncStatus.syncing ? 'Syncing...' : syncFailed ? 'Sync Failed' : 'Sync Now') : 'Set up Sync'"
                >
                    <!-- Success checkmark -->
                    <svg v-if="justSynced" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="sync-check">
                        <polyline points="20 6 9 17 4 12"/>
                    </svg>
                    <!-- Error X mark -->
                    <svg v-else-if="syncFailed" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="sync-error">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                    <!-- Cloud icon with spin animation when syncing -->
                    <svg v-else width="18" height="18" viewBox="0 0 24 24" :class="{ 'spinning': syncStatus.syncing }">
                        <path d="M4 16.5A4.5 4.5 0 0 1 5.5 8a5.5 5.5 0 0 1 10.3-1.5 4.5 4.5 0 0 1 4.7 6.3A3.5 3.5 0 0 1 18 19H6.5A4.5 4.5 0 0 1 4 16.5z"
                            :fill="syncStatus.connected ? 'currentColor' : 'none'"
                            :fill-opacity="syncStatus.connected ? 0.15 : 0"
                            stroke="currentColor"
                            stroke-width="1.5"
                            stroke-linecap="round"
                            stroke-linejoin="round"/>
                    </svg>
                </button>
                <!-- Menu Button (for Windows) -->
                <div class="menu-dropdown" :class="{ open: showMenuDropdown }">
                    <button class="menu-btn" @click.stop="showMenuDropdown = !showMenuDropdown" title="Menu">
                        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
                            <rect y="2" width="16" height="2" rx="1"/>
                            <rect y="7" width="16" height="2" rx="1"/>
                            <rect y="12" width="16" height="2" rx="1"/>
                        </svg>
                    </button>
                    <div v-if="showMenuDropdown" class="dropdown-content" @click.stop>
                        <div class="dropdown-section">
                            <div class="dropdown-label">{{ t('menu.export') }}</div>
                            <button @click="exportJSON">{{ t('menu.json') }}</button>
                            <button v-if="!isMobile" @click="exportToPDF">{{ t('menu.pdf') }}</button>
                            <button @click="exportToMarkdown">{{ t('menu.markdown') }}</button>
                            <button @click="openExportModal" class="range-export-btn">{{ t('export.rangeExport') }}</button>
                        </div>
                        <div class="dropdown-divider"></div>
                        <div class="dropdown-section">
                            <div class="dropdown-label">{{ t('menu.import') }}</div>
                            <button @click="importJSON">{{ t('menu.fromJson') }}</button>
                        </div>
                    </div>
                </div>
                <button class="settings-btn" @click="openSettings" :title="t('titlebar.settings')">⚙</button>
            </div>
        </div>

        <!-- Content Area -->
        <div class="content-wrapper" :class="{ 'split-mode': isSplitMode, 'mobile': isMobile }">
            <div class="left-wrapper" :class="{ 'drawer': isMobile, 'drawer-open': isMobile && drawerOpen }" :style="{ ...(!isMobile ? { width: leftWidth + 'px' } : {}), ...panelBgStyle }">
                <!-- Mobile Drawer Header -->
                <div v-if="isMobile" class="drawer-header" :class="{ 'dark-mode': darkTheme }">
                    <div class="drawer-header-content">
                        <button class="drawer-close-btn" @click="drawerOpen = false">
                            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="18" y1="6" x2="6" y2="18"/>
                                <line x1="6" y1="6" x2="18" y2="18"/>
                            </svg>
                        </button>
                    </div>
                    <!-- Mobile View Tabs: Calendar, Tags -->
                    <div class="drawer-tabs">
                        <button
                            :class="{ active: mobileView === 'calendar' }"
                            @click="mobileView = 'calendar'; selectedTagId = null; loadScheduleEvents()"
                        >
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="3" y="4" width="18" height="18" rx="2"/>
                                <line x1="3" y1="10" x2="21" y2="10"/>
                            </svg>
                            {{ t('drawer.calendar') }}
                        </button>
                        <button
                            :class="{ active: mobileView === 'tags' }"
                            @click="mobileView = 'tags'"
                        >
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
                            </svg>
                            {{ t('drawer.tags') }}
                        </button>
                    </div>
                </div>

                <!-- Desktop Search Input -->
                <input
                    v-if="!isMobile"
                    ref="searchInput"
                    v-model="searchText"
                    :placeholder="t('search.placeholder')"
                    @input="filterEntries"
                    class="search-input"
                    :class="{ 'dark-mode': darkTheme }"
                />

                <!-- Mobile Tags View -->
                <div v-if="isMobile && mobileView === 'tags'" class="mobile-tags-view">
                    <TagBar
                        :tags="tags"
                        :selected-tag-id="selectedTagId"
                        :dark-theme="darkTheme"
                        :t="t"
                        @select="filterByTagMobile"
                        @manage="showTagManager = true"
                        class="mobile-tag-bar"
                    />
                    <!-- Filtered entries by tag -->
                    <div v-if="selectedTagId" class="mobile-tag-entries">
                        <div class="tag-entries-header">
                            <span>{{ t('tags.entriesCount', { count: filteredEntries.length }) }}</span>
                            <button @click="selectedTagId = null; filteredEntries = []" class="clear-filter-btn">{{ t('tags.clear') }}</button>
                        </div>
                        <div
                            v-for="entry in filteredEntries"
                            :key="entry.date"
                            class="mobile-entry-card"
                            @click="selectEntry(entry)"
                        >
                            <div class="entry-card-header">
                                <span class="entry-card-date">{{ formatEntryDate(entry.date) }}</span>
                            </div>
                            <p class="entry-card-preview">{{ entry.content.substring(0, 80) }}...</p>
                        </div>
                    </div>
                    <div v-else class="tags-placeholder">
                        <!-- Minimal tag illustration -->
                        <svg width="80" height="80" viewBox="0 0 80 80" fill="none" stroke="currentColor" stroke-width="1" opacity="0.2">
                            <path d="M55 35L35 55a5 5 0 0 1-7 0L15 42V20h22l18 18a5 5 0 0 1 0 7z"/>
                            <circle cx="28" cy="33" r="4"/>
                            <line x1="45" y1="48" x2="55" y2="58"/>
                        </svg>
                        <p>{{ t('tags.selectTag') }}</p>
                    </div>
                </div>

                <!-- Mobile Calendar View -->
                <div v-if="isMobile && mobileView === 'calendar'" class="mobile-calendar-view">
                    <!-- Current Entry Info Card -->
                    <div class="current-entry-card" :class="{ 'dark-mode': darkTheme }">
                        <div class="entry-info">
                            <span class="entry-date-label">{{ t('schedule.selectedDate') }}</span>
                            <span class="entry-date-value">{{ formatEntryDate(date) }}</span>
                        </div>
                        <div v-if="currentEntryTags.length > 0" class="entry-current-tags">
                            <span
                                v-for="tag in currentEntryTags"
                                :key="tag.id"
                                class="entry-tag-pill"
                                :style="{ background: tag.color + '20', color: tag.color }"
                            >{{ tag.name }}</span>
                        </div>
                    </div>

                    <VDatePicker
                        borderless
                        transparent
                        :is-dark="darkTheme"
                        v-model="date"
                        color="teal"
                        class="mobile-date-picker"
                        :attributes="calendarAttributes"
                        @update:model-value="onDateChangeWithSchedule"
                    />

                    <!-- Schedule Header -->
                    <div class="schedule-header">
                        <h2>{{ formatEntryDate(date) }}</h2>
                        <button class="add-event-btn" @click="openNewEventModal">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="12" y1="5" x2="12" y2="19"/>
                                <line x1="5" y1="12" x2="19" y2="12"/>
                            </svg>
                            Add
                        </button>
                    </div>

                    <!-- Events List -->
                    <div class="events-list" v-if="sortedScheduleEvents.length > 0">
                        <div
                            v-for="event in sortedScheduleEvents"
                            :key="event.id"
                            class="event-card"
                            @click="editScheduleEvent(event)"
                        >
                            <span class="event-time">{{ event.time }}</span>
                            <span class="event-title">{{ event.title }}</span>
                        </div>
                    </div>
                    <div v-else class="no-events">
                        <p>{{ t('schedule.noSchedules') }}</p>
                    </div>
                </div>

                <!-- Event Modal Component -->
                <EventModal
                    :show="showEventModal"
                    :dark-theme="darkTheme"
                    :editing-event="editingEvent"
                    :initial-date="date"
                    :categories="eventCategories"
                    :t="t"
                    @close="closeEventModal"
                    @save="handleEventSave"
                    @delete="handleEventDelete"
                    @update-categories="eventCategories = $event"
                />

                <!-- Desktop Tag Bar -->
                <TagBar
                    v-if="!isMobile"
                    :tags="tags"
                    :selected-tag-id="selectedTagId"
                    :dark-theme="darkTheme"
                    @select="filterByTag"
                    @manage="showTagManager = true"
                />

                <!-- Desktop Calendar component -->
                <div v-if="!isMobile && !searchText && !selectedTagId" class="date-picker-container">
                    <VDatePicker
                        borderless
                        transparent
                        :is-dark="darkTheme"
                        v-model="date"
                        color="orange"
                        class="date-picker"
                        :attributes="calendarAttributes"
                        @update:model-value="onDateChange"
                    />
                </div>

                <!-- Desktop Search/Filter results -->
                <div v-if="!isMobile && (searchText || selectedTagId)" class="search-results">
                    <div v-if="filteredEntries.length === 0" style="padding: 20px; text-align: center; color: #999;">
                        <p v-if="entries.length === 0">{{ t('search.noDiariesYet') }}</p>
                        <p v-else>{{ t('search.noMatchingDiaries') }}</p>
                    </div>
                    <div
                        v-for="entry in filteredEntries"
                        :key="entry.date"
                        class="entry-preview"
                        @click="selectEntry(entry)"
                        :style="{ color: darkTheme ? '#fff' : '#000' }"
                    >
                        <div class="entry-preview-header">
                            <span class="entry-preview-date">{{ entry.date }}</span>
                            <div class="entry-preview-tags">
                                <span
                                    v-for="tag in entry.tags"
                                    :key="tag.id"
                                    class="entry-tag-dot"
                                    :style="{ background: tag.color }"
                                    :title="tag.name"
                                ></span>
                            </div>
                        </div>
                        <p>{{ entry.content.substring(0, 100) }}...</p>
                    </div>
                </div>

                <!-- Theme toggle button (Desktop only) -->
                <label v-if="!isMobile" class="toggleThemeBtn cursor-pointer">
                    <input
                        type="checkbox"
                        v-model="darkTheme"
                        @change="toggledarkTheme"
                        class="sr-only"
                    />
                    <div class="relative w-16 h-8 rounded-full transition"
                         :class="darkTheme ? 'bg-gray-700' : 'bg-gray-300'">
                        <div class="absolute left-1 top-1 w-6 h-6 bg-white rounded-full transition-transform flex items-center justify-center shadow-md"
                             :class="darkTheme ? 'translate-x-8' : 'translate-x-0'">
                            <span class="text-base">{{ darkTheme ? '☾' : '☼' }}</span>
                        </div>
                    </div>
                </label>

                <!-- Mobile Theme Toggle in Drawer Footer -->
                <div v-if="isMobile" class="drawer-footer">
                    <label class="mobile-theme-toggle">
                        <span class="theme-label">{{ darkTheme ? 'Dark Mode' : 'Light Mode' }}</span>
                        <div class="toggle-switch" :class="{ active: darkTheme }" @click="darkTheme = !darkTheme; toggledarkTheme()">
                            <div class="toggle-thumb">
                                <span>{{ darkTheme ? '🌙' : '☀️' }}</span>
                            </div>
                        </div>
                    </label>
                </div>
            </div>

            <div class="right-wrapper" :class="{ 'split-editor': isSplitMode }" :style="panelBgStyle">
                <!-- Milkdown Markdown Editor (Typora-like) -->
                <MarkdownEditor
                    ref="markdownEditor"
                    v-model="text"
                    :theme="darkTheme ? 'dark' : 'light'"
                    :hide-toggle="isSplitMode"
                    :diary-dir="storagePath"
                />

                <!-- Entry Tag Selector (floating bottom-left) -->
                <EntryTagSelector
                    v-if="!isSplitMode"
                    :available-tags="tags"
                    :selected-tags="currentEntryTags"
                    :dark-theme="darkTheme"
                    :t="t"
                    @update="saveEntryTags"
                    class="floating-tags"
                />

                <!-- Zen Mode Button (Desktop only) -->
                <button
                    v-if="!isSplitMode && !isMobile"
                    @click="toggleSplitMode"
                    class="zen-mode-btn"
                    :class="{ 'dark-mode': darkTheme }"
                    title="Zen Mode (⌘⇧P)"
                >
                    ◎
                </button>
            </div>

            <!-- Split Preview Panel (always in DOM for smooth animation) -->
            <div v-show="isSplitMode" class="split-preview-wrapper" :class="{ 'dark-mode': darkTheme, 'visible': isSplitMode }">
                <div class="split-preview-content">
                    <div v-html="splitPreviewHtml"></div>
                </div>
                <button class="split-close-btn" @click="isSplitMode = false" title="Exit Split Mode (Esc)">
                    ✕
                </button>
            </div>
        </div>

    </div>

    <!-- Tag Manager Modal (outside main-wrapper for proper fixed positioning) -->
    <TagManager
        v-if="showTagManager"
        :tags="tags"
        :dark-theme="darkTheme"
        :t="t"
        @close="showTagManager = false"
        @create="handleCreateTag"
        @update="handleUpdateTag"
        @delete="handleDeleteTag"
    />

    <!-- Mobile Bottom Navigation -->
    <MobileBottomNav
        v-if="isMobile && !isLocked"
        :dark-theme="darkTheme"
        :drawer-open="drawerOpen"
        :mobile-view="mobileView"
        :t="t"
        @nav="handleMobileNav"
        @open-settings="openSettings"
    />

    <!-- Export Range Modal -->
    <div v-if="showExportModal" class="export-modal-overlay" @click.self="showExportModal = false">
        <div class="export-modal" :class="{ 'dark-mode': darkTheme }">
            <div class="export-modal-header">
                <h2>{{ t('export.rangeExport') }}</h2>
                <button @click="showExportModal = false" class="close-btn">×</button>
            </div>
            <div class="export-modal-content">
                <!-- Quick Select Buttons -->
                <div class="export-quick-select">
                    <button @click="setExportThisWeek" :class="{ active: isThisWeekSelected }">
                        {{ t('export.thisWeek') }}
                    </button>
                    <button @click="setExportThisMonth" :class="{ active: isThisMonthSelected }">
                        {{ t('export.thisMonth') }}
                    </button>
                </div>

                <!-- Date Range Inputs -->
                <div class="export-date-range">
                    <div class="date-input-group">
                        <label>{{ t('export.startDate') }}</label>
                        <input type="date" v-model="exportRangeStart" />
                    </div>
                    <div class="date-input-group">
                        <label>{{ t('export.endDate') }}</label>
                        <input type="date" v-model="exportRangeEnd" />
                    </div>
                </div>

                <!-- Entry Count Preview -->
                <div class="export-entry-count" v-if="exportRangeStart && exportRangeEnd">
                    {{ entriesInRangeCount > 0 ? t('export.entriesInRange', { count: entriesInRangeCount }) : t('export.noEntriesInRange') }}
                </div>

                <!-- Format Selection -->
                <div class="export-format-select">
                    <label>{{ t('export.format') }}</label>
                    <div class="format-options">
                        <button @click="exportFormat = 'json'" :class="{ active: exportFormat === 'json' }">JSON</button>
                        <button v-if="!isMobile" @click="exportFormat = 'pdf'" :class="{ active: exportFormat === 'pdf' }">PDF</button>
                        <button @click="exportFormat = 'markdown'" :class="{ active: exportFormat === 'markdown' }">Markdown</button>
                    </div>
                </div>

                <!-- Export Button -->
                <button
                    class="export-btn"
                    @click="exportWithRange"
                    :disabled="!exportRangeStart || !exportRangeEnd || entriesInRangeCount === 0"
                >
                    {{ t('export.export') }}
                </button>
            </div>
        </div>
    </div>

    <!-- Export Loading Overlay (outside main-wrapper for proper fixed positioning) -->
    <div v-if="isExporting" class="export-overlay" :class="{ 'dark-theme': darkTheme }">
        <div class="export-spinner-container">
            <div class="export-spinner"></div>
            <div class="export-text">{{ t('export.exporting', { type: exportType }) }}</div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';
import { convertFileSrc } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { platform } from '@tauri-apps/plugin-os';
import { marked } from 'marked';
import katex from 'katex';
import 'katex/dist/katex.min.css';

// Configure marked
marked.setOptions({
    breaks: true,
    gfm: true,
});

import MarkdownEditor from '../components/MarkdownEditor.vue';
import TagBar from '../components/TagBar.vue';
import TagManager from '../components/TagManager.vue';
import EntryTagSelector from '../components/EntryTagSelector.vue';
import MobileTopBar from '../components/MobileTopBar.vue';
import MobileBottomNav from '../components/MobileBottomNav.vue';
import LockScreen from '../components/LockScreen.vue';
import SettingsModal from '../components/SettingsModal.vue';
import EventModal from '../components/EventModal.vue';
import SyncSettings from '../components/SyncSettings.vue';
import SyncStatus from '../components/SyncStatus.vue';
import { t, setLanguage, initLanguage, getAvailableLanguages } from '../i18n';

export default {
    components: {
        MarkdownEditor,
        TagBar,
        TagManager,
        EntryTagSelector,
        MobileTopBar,
        MobileBottomNav,
        LockScreen,
        SettingsModal,
        EventModal,
        SyncSettings,
        SyncStatus
    },
    data() {
        const today = this.getFormattedDate(new Date());
        return {
            searchText: "",
            date: today,
            text: `${today}\n`,
            entries: [],
            filteredEntries: [],
            darkTheme: false,
            // Settings
            showSettings: false,
            storagePath: '',
            config: {
                bgType: 'preset',
                bgValue: 'teal',
                bgFit: 'cover',
                bgOpacity: 1,
                panelOpacity: 0.45,
                diaryDir: null,
                leftWidth: 320,
                windowWidth: 1200,
                windowHeight: 800,
                language: 'en'
            },
            availableLanguages: getAvailableLanguages(),
            // Password
            isLocked: false,
            passwordInput: '',
            lockError: '',
            passwordEnabled: false,
            newPassword: '',
            confirmPassword: '',
            // Preset themes
            presetThemes: [
                { name: 'blue', label: 'Blue', light: '#e8f4f8', dark: '#1a1f2e' },
                { name: 'green', label: 'Green', light: '#e8f8ed', dark: '#1a2e1f' },
                { name: 'purple', label: 'Purple', light: '#f0e8f8', dark: '#251a2e' },
                { name: 'orange', label: 'Orange', light: '#f8f0e8', dark: '#2e251a' },
                { name: 'gray', label: 'Gray', light: '#f0f0f0', dark: '#1f1f1f' },
            ],
            // Layout
            leftWidth: 320,
            resizeUnlisten: null,
            resizeTimer: null,
            // Tags
            tags: [],
            selectedTagId: null,
            showTagManager: false,
            currentEntryTags: [],
            // Split mode
            isSplitMode: false,
            // Settings tab
            settingsTab: 'general',
            // Menu dropdown
            showMenuDropdown: false,
            // Settings preview (temporary config for preview)
            tempConfig: null,
            // Export loading state
            isExporting: false,
            exportType: '',
            // Export range modal
            showExportModal: false,
            exportRangeStart: '',
            exportRangeEnd: '',
            exportFormat: 'json',
            // Mobile platform detection
            isMobile: false,
            drawerOpen: false,
            // Mobile view state: 'calendar', 'tags'
            mobileView: 'calendar',
            // Touch gesture tracking
            touchStartX: 0,
            touchStartY: 0,
            // Auto-save state (Apple style)
            isSaving: false,
            justSaved: false,
            autoSaveTimer: null,
            lastSavedText: '',
            // Schedule state
            scheduleEvents: [],
            showEventModal: false,
            editingEvent: null,
            scheduleWeekStart: null, // Date object for the start of current week
            eventCategories: [
                { id: 'exercise', name: 'Exercise', color: 'var(--event-meeting)' },
                { id: 'study', name: 'Study', color: 'var(--event-hangout)' },
                { id: 'meditation', name: 'Meditation', color: 'var(--event-cooking)' }
            ],
            // Alarm state
            firedAlarms: new Set(),      // Track fired alarms to avoid duplicates
            alarmCheckInterval: null,     // Interval reference for alarm scheduler
            // Sync state
            syncStatus: {
                connected: false,
                syncing: false,
                lastSync: null,
                error: null,
                accountEmail: null
            },
            justSynced: false,
            syncFailed: false
        };
    },
    computed: {
        // Mobile date display
        currentWeekday() {
            const dateObj = new Date(this.date + 'T00:00:00');
            return dateObj.toLocaleDateString('en-US', { weekday: 'short' });
        },
        currentDateDisplay() {
            const dateObj = new Date(this.date + 'T00:00:00');
            return dateObj.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' });
        },
        currentYear() {
            const dateObj = new Date(this.date + 'T00:00:00');
            return dateObj.getFullYear();
        },
        // Schedule week days
        weekDays() {
            const start = this.scheduleWeekStart || this.getWeekStart(new Date());
            const days = [];
            const today = this.getFormattedDate(new Date());
            for (let i = 0; i < 7; i++) {
                const d = new Date(start);
                d.setDate(d.getDate() + i);
                const dateStr = this.getFormattedDate(d);
                days.push({
                    date: dateStr,
                    dayName: d.toLocaleDateString('en-US', { weekday: 'short' }),
                    dayNumber: d.getDate(),
                    isToday: dateStr === today,
                    isSelected: dateStr === this.date
                });
            }
            return days;
        },
        // Sorted schedule events for current date
        sortedScheduleEvents() {
            return [...this.scheduleEvents].sort((a, b) => {
                const [aH, aM] = a.time.split(':').map(Number);
                const [bH, bM] = b.time.split(':').map(Number);
                return (aH * 60 + aM) - (bH * 60 + bM);
            });
        },
        // Base style for main wrapper (colors and presets, not images)
        bgBaseStyle() {
            const { bgType, bgValue } = this.config;
            if (bgType === 'color' && bgValue) {
                return { backgroundColor: bgValue };
            }
            if (bgType === 'preset' && bgValue) {
                const preset = this.presetThemes.find(p => p.name === bgValue);
                if (preset) {
                    return { backgroundColor: this.darkTheme ? preset.dark : preset.light };
                }
            }
            return {};
        },
        // Style for background image overlay (with opacity support)
        bgImageStyle() {
            const { bgType, bgValue, bgFit, bgOpacity } = this.config;
            if (bgType === 'image' && bgValue) {
                const imgUrl = convertFileSrc(bgValue);
                return {
                    backgroundImage: `url(${imgUrl})`,
                    backgroundSize: bgFit === 'fill' ? '100% 100%' : (bgFit || 'cover'),
                    backgroundPosition: 'center',
                    backgroundRepeat: 'no-repeat',
                    opacity: bgOpacity !== undefined ? bgOpacity : 1
                };
            }
            return {};
        },
        // Dynamic panel background style (for image backgrounds)
        panelBgStyle() {
            if (this.config.bgType === 'image' && this.config.bgValue) {
                const opacity = this.config.panelOpacity !== undefined ? this.config.panelOpacity : 0.45;
                if (this.darkTheme) {
                    return { background: `rgba(30, 35, 50, ${opacity})` };
                } else {
                    return { background: `rgba(255, 255, 255, ${opacity})` };
                }
            }
            return {};
        },
        // Split mode preview HTML
        splitPreviewHtml() {
            if (!this.text) return '';
            let content = this.text;

            // Store math expressions with placeholders
            const mathExpressions = [];
            let mathIndex = 0;

            // Replace display math $$ ... $$
            content = content.replace(/\$\$([\s\S]+?)\$\$/g, (_, formula) => {
                try {
                    const rendered = katex.renderToString(formula.trim(), {
                        displayMode: true,
                        throwOnError: false
                    });
                    const placeholder = `<!--MATH_BLOCK_${mathIndex}-->`;
                    mathExpressions.push({ id: mathIndex, html: rendered, isBlock: true });
                    mathIndex++;
                    return placeholder;
                } catch (e) {
                    return `<div class="math-error">Math Error: ${e.message}</div>`;
                }
            });

            // Replace inline math $ ... $
            content = content.replace(/\$([^\$\n]+?)\$/g, (_, formula) => {
                try {
                    const rendered = katex.renderToString(formula.trim(), {
                        displayMode: false,
                        throwOnError: false
                    });
                    const placeholder = `<!--MATH_INLINE_${mathIndex}-->`;
                    mathExpressions.push({ id: mathIndex, html: rendered, isBlock: false });
                    mathIndex++;
                    return placeholder;
                } catch (e) {
                    return `<span class="math-error">Math Error: ${e.message}</span>`;
                }
            });

            // Convert image paths to Tauri asset URLs
            // 1. Handle relative paths (images/xxx.png) - new format for cross-device sync
            if (this.storagePath) {
                content = content.replace(/!\[([^\]]*)\]\((images\/[^)]+)\)/g, (match, alt, relativePath) => {
                    const fullPath = `${this.storagePath}/${relativePath}`;
                    return `![${alt}](${convertFileSrc(fullPath)})`;
                });
            }

            // 2. Handle absolute paths starting with / (legacy format, for backward compatibility)
            content = content.replace(/!\[([^\]]*)\]\((\/.+?)\)/g, (match, alt, path) => {
                return `![${alt}](${convertFileSrc(path)})`;
            });

            // Render markdown
            let html = marked(content);

            // Restore math expressions
            mathExpressions.forEach(({ id, html: mathHtml, isBlock }) => {
                if (isBlock) {
                    html = html.replace(`<!--MATH_BLOCK_${id}-->`, `<div class="math-block">${mathHtml}</div>`);
                } else {
                    html = html.replace(`<!--MATH_INLINE_${id}-->`, mathHtml);
                }
            });

            return html;
        },
        // Calendar attributes for showing tag dots on dates
        calendarAttributes() {
            const attrs = [];
            for (const entry of this.entries) {
                if (entry.tags && entry.tags.length > 0) {
                    // Show all tags as dots
                    entry.tags.forEach((tag, index) => {
                        attrs.push({
                            key: `${entry.date}-tag-${index}`,
                            dates: new Date(entry.date + 'T00:00:00'),
                            dot: {
                                style: {
                                    backgroundColor: tag.color
                                }
                            }
                        });
                    });
                }
            }
            return attrs;
        },
        // Export range computed properties
        entriesInRangeCount() {
            if (!this.exportRangeStart || !this.exportRangeEnd) return 0;
            return this.entries.filter(entry =>
                entry.date >= this.exportRangeStart && entry.date <= this.exportRangeEnd
            ).length;
        },
        isThisWeekSelected() {
            const { start, end } = this.getThisWeekRange();
            return this.exportRangeStart === start && this.exportRangeEnd === end;
        },
        isThisMonthSelected() {
            const { start, end } = this.getThisMonthRange();
            return this.exportRangeStart === start && this.exportRangeEnd === end;
        }
    },
    watch: {
        // Auto-save when text changes (Apple style)
        text(newVal, oldVal) {
            if (this.isMobile && newVal !== oldVal) {
                this.triggerAutoSave();
            }
        }
    },
    methods: {
        // i18n translation helper
        t(key, params = {}) {
            return t(key, params);
        },
        onLanguageChange() {
            setLanguage(this.config.language);
            // Force Vue to re-render by updating a reactive property
            this.$forceUpdate();
        },
        getFormattedDate(date) {
            const year = date.getFullYear();
            const month = String(date.getMonth() + 1).padStart(2, "0");
            const day = String(date.getDate()).padStart(2, "0");
            return `${year}-${month}-${day}`;
        },
        async onDateChange(newDate) {
            this.date = this.getFormattedDate(newDate);
            await this.loadDiaryEntry();
            await this.loadCurrentEntryTags();
            // Close drawer on mobile after selecting date
            if (this.isMobile) {
                this.drawerOpen = false;
            }
        },
        async onDateChangeWithSchedule(newDate) {
            this.date = this.getFormattedDate(newDate);
            await this.loadDiaryEntry();
            await this.loadCurrentEntryTags();
            await this.loadScheduleEvents();
        },
        async loadDiaryEntry() {
            try {
                const entry = await invoke('load_diary', { date: this.date });
                this.text = entry || `${this.date}\n`;
            } catch (error) {
                console.error('Failed to load diary:', error);
                this.text = `${this.date}\n`;
            }
        },
        async saveDiary() {
            try {
                await invoke('save_diary', { date: this.date, content: this.text });
                await this.loadAllEntries();
                console.log('Diary saved');
            } catch (error) {
                console.error('Save failed:', error);
            }
        },
        // Auto-save with debounce (Apple style)
        triggerAutoSave() {
            // Clear existing timer
            if (this.autoSaveTimer) {
                clearTimeout(this.autoSaveTimer);
            }
            // Don't save if content hasn't changed
            if (this.text === this.lastSavedText) {
                return;
            }
            // Set new timer (save after 1.5 seconds of inactivity)
            this.autoSaveTimer = setTimeout(() => {
                this.performAutoSave();
            }, 1500);
        },
        async performAutoSave() {
            if (this.text === this.lastSavedText) return;

            this.isSaving = true;
            try {
                await invoke('save_diary', { date: this.date, content: this.text });
                this.lastSavedText = this.text;
                await this.loadAllEntries();

                // Show saved indicator
                this.isSaving = false;
                this.justSaved = true;
                setTimeout(() => {
                    this.justSaved = false;
                }, 2000);
            } catch (error) {
                console.error('Auto-save failed:', error);
                this.isSaving = false;
            }
        },
        async loadAllEntries() {
            try {
                this.entries = await invoke('get_all_diaries');
            } catch (error) {
                console.error('Failed to load entries:', error);
            }
        },
        filterEntries() {
            if (!this.searchText && !this.selectedTagId) {
                this.filteredEntries = [];
                return;
            }

            let filtered = [...this.entries];

            // Filter by tag first
            if (this.selectedTagId) {
                filtered = filtered.filter(entry =>
                    entry.tags?.some(t => t.id === this.selectedTagId)
                );
            }

            // Then filter by search text
            if (this.searchText) {
                const searchLower = this.searchText.toLowerCase();
                filtered = filtered.filter(entry =>
                    entry.content.toLowerCase().includes(searchLower) ||
                    entry.date.includes(searchLower)
                );
            }

            this.filteredEntries = filtered;
            console.log('Filter results:', filtered.length, 'entries');
        },
        selectEntry(entry) {
            this.date = entry.date;
            this.text = entry.content;
            this.searchText = "";
            this.loadCurrentEntryTags();
            // Close drawer on mobile after selecting entry
            if (this.isMobile) {
                this.drawerOpen = false;
            }
        },
        // Tag methods
        async loadTags() {
            try {
                this.tags = await invoke('get_all_tags');
            } catch (error) {
                console.error('Failed to load tags:', error);
            }
        },
        async loadCurrentEntryTags() {
            try {
                this.currentEntryTags = await invoke('get_entry_tags', { date: this.date });
            } catch (error) {
                console.error('Failed to load entry tags:', error);
            }
        },
        async loadAllEntriesWithTags() {
            try {
                const entries = await invoke('get_all_diaries');
                // Load tags for each entry
                for (const entry of entries) {
                    entry.tags = await invoke('get_entry_tags', { date: entry.date });
                }
                this.entries = entries;
            } catch (error) {
                console.error('Failed to load entries with tags:', error);
            }
        },
        filterByTag(tagId) {
            this.selectedTagId = tagId;
            this.filterEntries();
        },
        filterByTagMobile(tagId) {
            this.selectedTagId = tagId;
            this.filterEntries();
            // Don't close drawer on mobile when filtering by tag
        },
        formatEntryDate(dateStr) {
            const dateObj = new Date(dateStr + 'T00:00:00');
            return dateObj.toLocaleDateString('en-US', {
                weekday: 'short',
                month: 'short',
                day: 'numeric',
                year: 'numeric'
            });
        },
        formatShortDate(dateStr) {
            const dateObj = new Date(dateStr + 'T00:00:00');
            return dateObj.toLocaleDateString('en-US', {
                month: 'short',
                day: 'numeric'
            });
        },
        async handleCreateTag({ name, color }) {
            try {
                await invoke('create_tag', { name, color });
                await this.loadTags();
            } catch (error) {
                console.error('Failed to create tag:', error);
                alert('Failed to create tag: ' + error);
            }
        },
        async handleUpdateTag({ id, name, color }) {
            try {
                await invoke('update_tag', { id, name, color });
                await this.loadTags();
                await this.loadCurrentEntryTags();
            } catch (error) {
                console.error('Failed to update tag:', error);
                alert('Failed to update tag: ' + error);
            }
        },
        async handleDeleteTag(id) {
            try {
                await invoke('delete_tag', { id });
                await this.loadTags();
                await this.loadCurrentEntryTags();
                if (this.selectedTagId === id) {
                    this.selectedTagId = null;
                }
                await this.loadAllEntriesWithTags();
            } catch (error) {
                console.error('Failed to delete tag:', error);
                alert('Failed to delete tag: ' + error);
            }
        },
        async saveEntryTags(tagIds) {
            try {
                await invoke('set_entry_tags', { date: this.date, tagIds });
                await this.loadCurrentEntryTags();
                await this.loadAllEntriesWithTags();
            } catch (error) {
                console.error('Failed to save entry tags:', error);
            }
        },
        toggledarkTheme() {
            console.log('Theme toggled:', this.darkTheme);
        },
        // Global keyboard shortcuts
        handleGlobalShortcut(event) {
            const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
            const cmdKey = isMac ? event.metaKey : event.ctrlKey;

            // Cmd+S - Save
            if (cmdKey && event.key === 's') {
                event.preventDefault();
                this.saveDiary();
                return;
            }

            // Cmd+Shift+P - Toggle split mode
            if (cmdKey && event.shiftKey && event.key.toLowerCase() === 'p') {
                event.preventDefault();
                this.toggleSplitMode();
                return;
            }

            // Cmd+F - Focus search
            if (cmdKey && event.key === 'f') {
                event.preventDefault();
                if (!this.isSplitMode) {
                    this.$refs.searchInput?.focus();
                }
                return;
            }

            // Cmd+T or Cmd+0 - Go to today
            if (cmdKey && (event.key === 't' || event.key === '0')) {
                event.preventDefault();
                this.goToToday();
                return;
            }

            // Esc - Exit split mode or clear search
            if (event.key === 'Escape') {
                if (this.isSplitMode) {
                    this.isSplitMode = false;
                    return;
                }
                if (this.searchText || this.selectedTagId) {
                    this.searchText = '';
                    this.selectedTagId = null;
                    this.filteredEntries = [];
                    return;
                }
            }
        },
        // Go to today's diary
        goToToday() {
            const today = this.getFormattedDate(new Date());
            this.date = today;
            this.searchText = '';
            this.selectedTagId = null;
            this.loadDiaryEntry();
            this.loadCurrentEntryTags();
        },
        toggleSplitMode() {
            this.isSplitMode = !this.isSplitMode;
        },
        // Export range modal methods
        openExportModal() {
            this.showMenuDropdown = false;
            this.setExportThisWeek(); // Default to this week
            this.showExportModal = true;
        },
        getThisWeekRange() {
            const today = new Date();
            const dayOfWeek = today.getDay(); // 0 = Sunday
            const startOfWeek = new Date(today);
            startOfWeek.setDate(today.getDate() - dayOfWeek);
            const endOfWeek = new Date(startOfWeek);
            endOfWeek.setDate(startOfWeek.getDate() + 6);
            return {
                start: this.getFormattedDate(startOfWeek),
                end: this.getFormattedDate(endOfWeek)
            };
        },
        getThisMonthRange() {
            const today = new Date();
            const startOfMonth = new Date(today.getFullYear(), today.getMonth(), 1);
            const endOfMonth = new Date(today.getFullYear(), today.getMonth() + 1, 0);
            return {
                start: this.getFormattedDate(startOfMonth),
                end: this.getFormattedDate(endOfMonth)
            };
        },
        setExportThisWeek() {
            const { start, end } = this.getThisWeekRange();
            this.exportRangeStart = start;
            this.exportRangeEnd = end;
        },
        setExportThisMonth() {
            const { start, end } = this.getThisMonthRange();
            this.exportRangeStart = start;
            this.exportRangeEnd = end;
        },
        async exportWithRange() {
            this.showExportModal = false;
            this.isExporting = true;
            this.exportType = this.exportFormat.toUpperCase();

            try {
                if (this.exportFormat === 'json') {
                    await invoke('export_json_range', {
                        startDate: this.exportRangeStart,
                        endDate: this.exportRangeEnd
                    });
                } else if (this.exportFormat === 'pdf') {
                    await invoke('export_pdf_range', {
                        startDate: this.exportRangeStart,
                        endDate: this.exportRangeEnd
                    });
                    alert(this.t('alert.pdfExportSuccess'));
                } else if (this.exportFormat === 'markdown') {
                    await this.exportMarkdownRange();
                }
            } catch (error) {
                console.error('Export failed:', error);
                if (error !== 'No file selected' && error !== 'cancelled') {
                    alert('Export failed: ' + error);
                }
            } finally {
                this.isExporting = false;
                this.exportType = '';
            }
        },
        async exportMarkdownRange() {
            const { save } = await import('@tauri-apps/plugin-dialog');
            const { writeFile, copyFile, mkdir, exists } = await import('@tauri-apps/plugin-fs');

            // Filter entries by date range and sort oldest first
            const filteredEntries = this.entries
                .filter(entry => entry.date >= this.exportRangeStart && entry.date <= this.exportRangeEnd)
                .sort((a, b) => a.date.localeCompare(b.date));

            let markdownContent = '';
            const imagePaths = new Set();

            for (const entry of filteredEntries) {
                let content = entry.content;
                const firstLine = content.split('\n')[0].trim();

                if (firstLine === entry.date || /^\d{4}-\d{2}-\d{2}$/.test(firstLine)) {
                    content = content.split('\n').slice(1).join('\n').trim();
                }

                if (content) {
                    markdownContent += `## ${entry.date}\n\n${content}\n\n---\n\n`;
                    // Collect image paths
                    const imgMatches = content.match(/!\[.*?\]\((images\/[^)]+)\)/g) || [];
                    imgMatches.forEach(match => {
                        const pathMatch = match.match(/\((images\/[^)]+)\)/);
                        if (pathMatch) imagePaths.add(pathMatch[1]);
                    });
                }
            }

            // Remove trailing separator
            markdownContent = markdownContent.replace(/---\n\n$/, '');

            const filePath = await save({
                title: 'Export Markdown',
                defaultPath: `diaries_${this.exportRangeStart}_to_${this.exportRangeEnd}.md`,
                filters: [{ name: 'Markdown', extensions: ['md'] }]
            });

            if (filePath) {
                await writeFile(filePath, new TextEncoder().encode(markdownContent));

                // Copy images if any
                if (imagePaths.size > 0 && this.storagePath) {
                    const exportDir = filePath.substring(0, filePath.lastIndexOf('/') || filePath.lastIndexOf('\\'));
                    const imagesDir = `${exportDir}/images`;

                    if (!(await exists(imagesDir))) {
                        await mkdir(imagesDir, { recursive: true });
                    }

                    let copiedCount = 0;
                    for (const imgPath of imagePaths) {
                        try {
                            const srcPath = `${this.storagePath}/${imgPath}`;
                            const destPath = `${exportDir}/${imgPath}`;
                            await copyFile(srcPath, destPath);
                            copiedCount++;
                        } catch (e) {
                            console.error('Failed to copy image:', e);
                        }
                    }

                    if (copiedCount > 0) {
                        alert(this.t('alert.markdownExportImages', { count: copiedCount }));
                    } else {
                        alert(this.t('alert.markdownExportSuccess'));
                    }
                } else {
                    alert(this.t('alert.markdownExportSuccess'));
                }
            }
        },
        async exportJSON() {
            this.showMenuDropdown = false;
            this.isExporting = true;
            this.exportType = 'JSON';
            try {
                await invoke('export_json');
                console.log('JSON exported successfully');
            } catch (error) {
                console.error('Export JSON failed:', error);
                if (error !== 'No file selected' && error !== 'cancelled') {
                    alert('Failed to export JSON: ' + error);
                }
            } finally {
                this.isExporting = false;
                this.exportType = '';
            }
        },
        async importJSON() {
            this.showMenuDropdown = false;
            try {
                const result = await invoke('import_json');
                console.log('JSON imported:', result);
                alert(result);
                await this.loadAllEntries();
                await this.loadDiaryEntry();
            } catch (error) {
                console.error('Import JSON failed:', error);
                if (error !== 'No file selected' && error !== 'cancelled') {
                    alert('Failed to import JSON: ' + error);
                }
            }
        },
        async exportToPDF() {
            this.showMenuDropdown = false;
            this.isExporting = true;
            this.exportType = 'PDF';
            try {
                await invoke('export_pdf');
                alert('PDF exported successfully!');
            } catch (error) {
                console.error('PDF export error:', error);
                if (error !== 'No file selected' && error !== 'cancelled') {
                    alert('Failed to export PDF: ' + error);
                }
            } finally {
                this.isExporting = false;
                this.exportType = '';
            }
        },
        async exportToMarkdown() {
            this.showMenuDropdown = false;
            try {
                const { save } = await import('@tauri-apps/plugin-dialog');
                const { writeFile, copyFile, mkdir, exists } = await import('@tauri-apps/plugin-fs');

                // Build markdown content from all entries (oldest first)
                let markdownContent = '';
                const sortedEntries = [...this.entries].reverse();
                const imagePaths = new Set(); // Collect all image paths

                for (const entry of sortedEntries) {
                    let content = entry.content;
                    const firstLine = content.split('\n')[0].trim();

                    // Remove first line if it's a date to avoid duplication
                    if (firstLine === entry.date || /^\d{4}-\d{2}-\d{2}$/.test(firstLine)) {
                        content = content.split('\n').slice(1).join('\n').trim();
                    }

                    if (!content || content.trim() === '') {
                        continue;
                    }

                    // Find all local image paths in content
                    const imgRegex = /!\[([^\]]*)\]\((\/[^)]+)\)/g;
                    let match;
                    while ((match = imgRegex.exec(content)) !== null) {
                        imagePaths.add(match[2]);
                    }

                    markdownContent += `## ${entry.date}\n\n`;
                    markdownContent += content + '\n\n';
                }

                const filePath = await save({
                    title: 'Export diaries as Markdown',
                    defaultPath: 'diaries.md',
                    filters: [{
                        name: 'Markdown Files',
                        extensions: ['md']
                    }]
                });

                if (filePath) {
                    this.isExporting = true;
                    this.exportType = 'Markdown';

                    // Get the directory of the markdown file
                    const dirPath = filePath.substring(0, filePath.lastIndexOf('/'));
                    const imagesDir = `${dirPath}/images`;

                    // Copy images if any exist
                    if (imagePaths.size > 0) {
                        // Create images directory if it doesn't exist
                        if (!(await exists(imagesDir))) {
                            await mkdir(imagesDir, { recursive: true });
                        }

                        // Copy each image and update paths in content
                        for (const srcPath of imagePaths) {
                            const fileName = srcPath.substring(srcPath.lastIndexOf('/') + 1);
                            const destPath = `${imagesDir}/${fileName}`;

                            try {
                                await copyFile(srcPath, destPath);
                                // Update path in markdown to relative path
                                markdownContent = markdownContent.replace(
                                    new RegExp(`!\\[([^\\]]*)\\]\\(${srcPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}\\)`, 'g'),
                                    `![$1](./images/${fileName})`
                                );
                            } catch (e) {
                                console.warn('Failed to copy image:', srcPath, e);
                            }
                        }
                    }

                    // Convert string to Uint8Array for writeFile
                    const encoder = new TextEncoder();
                    const data = encoder.encode(markdownContent);
                    await writeFile(filePath, data);

                    this.isExporting = false;
                    this.exportType = '';

                    const imageCount = imagePaths.size;
                    if (imageCount > 0) {
                        alert(`Markdown exported successfully!\n${imageCount} images copied to ./images/`);
                    } else {
                        alert('Markdown exported successfully!');
                    }
                }
            } catch (error) {
                this.isExporting = false;
                this.exportType = '';
                console.error('Markdown export error:', error);
                alert('Failed to export Markdown: ' + error.message);
            }
        },
        // Settings methods
        async loadConfig() {
            try {
                const cfg = await invoke('get_config');
                this.config.bgType = cfg.bgType || 'preset';
                this.config.bgValue = cfg.bgValue || 'blue';
                this.config.bgFit = cfg.bgFit || 'cover';
                this.config.bgOpacity = cfg.bgOpacity !== undefined ? cfg.bgOpacity : 1;
                this.config.panelOpacity = cfg.panelOpacity !== undefined ? cfg.panelOpacity : 0.45;
                this.config.diaryDir = cfg.diaryDir;
                this.config.leftWidth = cfg.leftWidth || 320;
                this.config.windowWidth = cfg.windowWidth || 1200;
                this.config.windowHeight = cfg.windowHeight || 800;
                this.config.language = cfg.language || 'en';
                // Initialize i18n
                initLanguage(this.config.language);
                this.leftWidth = this.config.leftWidth;
                this.storagePath = await invoke('get_storage_path');
                this.passwordEnabled = await invoke('has_password');
                // Restore window size (desktop only)
                if (!this.isMobile) {
                    await this.restoreWindowSize();
                }
            } catch (error) {
                console.error('Failed to load config:', error);
            }
        },
        async restoreWindowSize() {
            // Skip on mobile - window API not available
            if (this.isMobile) return;

            try {
                // Clamp to valid range
                const width = Math.max(800, Math.min(2560, this.config.windowWidth || 1200));
                const height = Math.max(600, Math.min(1440, this.config.windowHeight || 800));

                const { getCurrentWindow } = await import('@tauri-apps/api/window');
                const win = getCurrentWindow();
                const { LogicalSize } = await import('@tauri-apps/api/dpi');
                await win.setSize(new LogicalSize(width, height));
            } catch (error) {
                console.error('Failed to restore window size:', error);
            }
        },
        async saveWindowSize() {
            // Skip on mobile - window API not available
            if (this.isMobile) return;

            try {
                const { getCurrentWindow } = await import('@tauri-apps/api/window');
                const win = getCurrentWindow();
                const size = await win.innerSize();
                const scaleFactor = await win.scaleFactor();
                // Convert physical to logical size
                this.config.windowWidth = Math.round(size.width / scaleFactor);
                this.config.windowHeight = Math.round(size.height / scaleFactor);
                await this.saveConfig();
            } catch (error) {
                console.error('Failed to save window size:', error);
            }
        },
        async saveConfig() {
            try {
                await invoke('save_config', {
                    diaryDir: this.config.diaryDir,
                    bgType: this.config.bgType,
                    bgValue: this.config.bgValue,
                    bgFit: this.config.bgFit,
                    bgOpacity: this.config.bgOpacity,
                    panelOpacity: this.config.panelOpacity,
                    leftWidth: this.leftWidth,
                    windowWidth: this.config.windowWidth,
                    windowHeight: this.config.windowHeight,
                    language: this.config.language
                });
            } catch (error) {
                console.error('Failed to save config:', error);
            }
        },
        async saveSettings() {
            try {
                // Apply window size if changed
                await this.applyWindowSize();
                await invoke('save_config', {
                    diaryDir: this.config.diaryDir,
                    bgType: this.config.bgType,
                    bgValue: this.config.bgValue,
                    bgFit: this.config.bgFit,
                    bgOpacity: this.config.bgOpacity,
                    panelOpacity: this.config.panelOpacity,
                    leftWidth: this.leftWidth,
                    windowWidth: this.config.windowWidth,
                    windowHeight: this.config.windowHeight,
                    language: this.config.language
                });
                // Clear temp config (changes are now saved)
                this.tempConfig = null;
                this.showSettings = false;
                alert(this.t('alert.settingsSaved'));
            } catch (error) {
                console.error('Failed to save settings:', error);
                alert('Failed to save settings: ' + error);
            }
        },
        async applyWindowSize() {
            try {
                // Clamp to valid range
                const width = Math.max(800, Math.min(2560, this.config.windowWidth || 1200));
                const height = Math.max(600, Math.min(1440, this.config.windowHeight || 800));
                this.config.windowWidth = width;
                this.config.windowHeight = height;

                const { getCurrentWindow } = await import('@tauri-apps/api/window');
                const win = getCurrentWindow();
                const { LogicalSize } = await import('@tauri-apps/api/dpi');
                await win.setSize(new LogicalSize(width, height));
            } catch (error) {
                console.error('Failed to apply window size:', error);
            }
        },
        async changeStoragePath() {
            try {
                const newPath = await invoke('change_storage_path');
                this.storagePath = newPath;
                this.config.diaryDir = newPath;
                alert('Storage location changed. Please restart the app.');
            } catch (error) {
                if (error !== 'No folder selected') {
                    alert('Failed to change path: ' + error);
                }
            }
        },
        switchBgType(type) {
            if (this.config.bgType === type) return;
            this.config.bgType = type;
            // Reset bgValue to appropriate default when switching types
            if (type === 'preset') {
                this.config.bgValue = 'teal';
            } else if (type === 'color') {
                this.config.bgValue = '#FDFCFA';
            } else if (type === 'image') {
                this.config.bgValue = '';
            }
        },
        async selectBgImage() {
            try {
                const { open } = await import('@tauri-apps/plugin-dialog');
                const { readFile } = await import('@tauri-apps/plugin-fs');

                const filePath = await open({
                    title: 'Select background image',
                    filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'] }]
                });

                if (filePath) {
                    const data = await readFile(filePath);
                    // Extract extension, handling both file paths and content URIs
                    let ext = 'png'; // default
                    const pathStr = String(filePath);
                    if (pathStr.includes('.')) {
                        const parts = pathStr.split('.');
                        const lastPart = parts[parts.length - 1].toLowerCase();
                        // Only use if it's a valid image extension
                        if (['png', 'jpg', 'jpeg', 'gif', 'webp'].includes(lastPart)) {
                            ext = lastPart;
                        }
                    }
                    const savedPath = await invoke('save_background_image', {
                        data: Array.from(data),
                        ext: ext
                    });
                    this.config.bgValue = savedPath;
                }
            } catch (error) {
                console.error('Failed to select image:', error);
                alert('Failed to select image: ' + error);
            }
        },
        // Password methods
        async checkPassword() {
            const hasPassword = await invoke('has_password');
            if (hasPassword) {
                this.isLocked = true;
                this.passwordEnabled = true;
            }
        },
        async handleUnlock(password) {
            this.passwordInput = password;
            await this.unlockApp();
        },
        async unlockApp() {
            try {
                const valid = await invoke('verify_password', { password: this.passwordInput });
                if (valid) {
                    this.isLocked = false;
                    this.lockError = '';
                    this.passwordInput = '';
                } else {
                    this.lockError = 'Incorrect password';
                }
            } catch (error) {
                this.lockError = 'Error verifying password';
            }
        },
        async togglePassword() {
            if (!this.passwordEnabled) {
                try {
                    await invoke('clear_password');
                    this.newPassword = '';
                    this.confirmPassword = '';
                } catch (error) {
                    console.error('Failed to clear password:', error);
                }
            }
        },
        async savePassword() {
            if (!this.newPassword) {
                alert('Please enter a password');
                return;
            }
            if (this.newPassword !== this.confirmPassword) {
                alert('Passwords do not match');
                return;
            }
            try {
                await invoke('set_password', { password: this.newPassword });
                alert('Password set successfully!');
                this.newPassword = '';
                this.confirmPassword = '';
            } catch (error) {
                alert('Failed to set password: ' + error);
            }
        },
        // Window control methods
        async closeWindow() {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().close();
        },
        async minimizeWindow() {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().minimize();
        },
        async toggleMaximize() {
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().toggleMaximize();
        },
        async startDrag(e) {
            // Only drag if clicking on the drag region, not on buttons
            if (e.target.closest('.control-btn') || e.target.closest('.settings-btn') || e.target.closest('.menu-dropdown')) return;
            const { getCurrentWindow } = await import('@tauri-apps/api/window');
            await getCurrentWindow().startDragging();
        },
        closeMenuDropdown() {
            this.showMenuDropdown = false;
        },
        handleMobileNav(view) {
            this.mobileView = view;
            this.drawerOpen = true;
        },
        openSettings() {
            // Save current config for restore if cancelled
            this.tempConfig = JSON.parse(JSON.stringify(this.config));
            this.showSettings = true;
        },
        closeSettings() {
            // Restore original config (cancel changes)
            if (this.tempConfig) {
                this.config = JSON.parse(JSON.stringify(this.tempConfig));
                this.tempConfig = null;
            }
            this.showSettings = false;
        },
        // Sync methods
        handleSyncClick() {
            // If connected, trigger immediate sync; otherwise open sync settings
            if (this.syncStatus.connected) {
                this.triggerQuickSync();
            } else {
                this.openSyncSettings();
            }
        },
        async triggerQuickSync() {
            if (this.syncStatus.syncing) return; // Already syncing

            this.syncStatus.syncing = true;
            this.justSynced = false;
            this.syncFailed = false;
            try {
                await invoke('start_sync');
                await this.loadSyncStatus();
                await this.loadAllEntriesWithTags();
                // Reload current diary entry to reflect any changes from sync
                await this.loadDiaryEntry();
                await this.loadCurrentEntryTags();
                // Show success checkmark
                this.justSynced = true;
                setTimeout(() => {
                    this.justSynced = false;
                }, 2000);
            } catch (error) {
                console.error('Quick sync failed:', error);
                // Show error X mark
                this.syncFailed = true;
                setTimeout(() => {
                    this.syncFailed = false;
                }, 3000);
            } finally {
                this.syncStatus.syncing = false;
            }
        },
        openSyncSettings() {
            this.settingsTab = 'sync';
            this.openSettings();
        },
        async loadSyncStatus() {
            try {
                this.syncStatus = await invoke('get_sync_status');
            } catch (error) {
                console.error('Failed to load sync status:', error);
            }
        },
        handleSyncCompleted() {
            // Reload entries after sync
            this.loadAllEntriesWithTags();
            this.loadSyncStatus();
        },
        // Date navigation methods for mobile swipe
        goToPreviousDate() {
            const current = new Date(this.date);
            current.setDate(current.getDate() - 1);
            this.date = this.getFormattedDate(current);
            this.loadDiaryEntry();
            this.loadCurrentEntryTags();
        },
        goToNextDate() {
            const current = new Date(this.date);
            current.setDate(current.getDate() + 1);
            this.date = this.getFormattedDate(current);
            this.loadDiaryEntry();
            this.loadCurrentEntryTags();
        },
        // Touch gesture handlers for drawer
        handleTouchStart(e) {
            this.touchStartX = e.touches[0].clientX;
            this.touchStartY = e.touches[0].clientY;
        },
        handleTouchEnd(e) {
            if (!this.isMobile) return;

            const touchEndX = e.changedTouches[0].clientX;
            const touchEndY = e.changedTouches[0].clientY;
            const deltaX = touchEndX - this.touchStartX;
            const deltaY = touchEndY - this.touchStartY;

            // When drawer is open, only allow left swipe to close
            if (this.drawerOpen) {
                if (deltaX < -50 && Math.abs(deltaY) < 50) {
                    this.drawerOpen = false;
                }
                return;
            }

            // Main screen: horizontal swipe to change date
            if (Math.abs(deltaX) > 50 && Math.abs(deltaY) < 50) {
                if (deltaX > 0) {
                    this.goToPreviousDate();  // Right swipe -> previous day
                } else {
                    this.goToNextDate();       // Left swipe -> next day
                }
            }
        },
        // ============== Schedule Methods ==============
        getWeekStart(date) {
            const d = new Date(date);
            const day = d.getDay();
            const diff = d.getDate() - day + (day === 0 ? -6 : 1); // Adjust for Monday start
            return new Date(d.setDate(diff));
        },
        prevWeek() {
            if (!this.scheduleWeekStart) {
                this.scheduleWeekStart = this.getWeekStart(new Date());
            }
            const newStart = new Date(this.scheduleWeekStart);
            newStart.setDate(newStart.getDate() - 7);
            this.scheduleWeekStart = newStart;
        },
        nextWeek() {
            if (!this.scheduleWeekStart) {
                this.scheduleWeekStart = this.getWeekStart(new Date());
            }
            const newStart = new Date(this.scheduleWeekStart);
            newStart.setDate(newStart.getDate() + 7);
            this.scheduleWeekStart = newStart;
        },
        async selectScheduleDate(dateStr) {
            this.date = dateStr;
            await this.loadScheduleEvents();
            await this.loadDiaryEntry();
            await this.loadCurrentEntryTags();
        },
        async onScheduleDateChange(newDate) {
            this.date = this.getFormattedDate(newDate);
            await this.loadScheduleEvents();
            await this.loadDiaryEntry();
            await this.loadCurrentEntryTags();
        },
        async loadScheduleEvents() {
            try {
                // Parse schedule lines from diary content
                // Format: "H:MM Title" (alarm data stored separately in localStorage)
                // Also handles legacy format: "H:MM [A:min:type] Title"
                const content = this.text || '';
                const lines = content.split('\n');
                const scheduleRegex = /^(\d{1,2}):(\d{2})\s+(.+)$/;
                const legacyAlarmRegex = /^\[A:\d+:[nsb]\]\s+/; // Strip old alarm format from title
                const events = [];

                // Load alarm settings from localStorage
                const alarmData = JSON.parse(localStorage.getItem('eventAlarms') || '{}');

                lines.forEach((line, index) => {
                    const match = line.trim().match(scheduleRegex);
                    if (match) {
                        const time = `${match[1]}:${match[2]}`;
                        // Strip legacy alarm format from title if present
                        let title = match[3].trim().replace(legacyAlarmRegex, '');
                        // Create alarm key for localStorage lookup
                        const alarmKey = `${this.date}:${time}:${title}`;
                        const alarm = alarmData[alarmKey] || {};

                        const event = {
                            id: `evt-${index}`,
                            time,
                            title,
                            alarmEnabled: alarm.enabled || false,
                            alarmMinutesBefore: alarm.minutesBefore ?? 10,
                            alarmType: alarm.type || 'notification'
                        };
                        events.push(event);
                    }
                });

                // Sort by time (numeric comparison for H:MM format)
                events.sort((a, b) => {
                    const [aH, aM] = a.time.split(':').map(Number);
                    const [bH, bM] = b.time.split(':').map(Number);
                    return (aH * 60 + aM) - (bH * 60 + bM);
                });
                this.scheduleEvents = events;
            } catch (error) {
                console.error('Failed to load events:', error);
                this.scheduleEvents = [];
            }
        },
        openNewEventModal() {
            this.editingEvent = null;
            this.showEventModal = true;
        },
        editScheduleEvent(event) {
            this.editingEvent = event;
            this.showEventModal = true;
        },
        closeEventModal() {
            this.showEventModal = false;
            this.editingEvent = null;
        },
        // Handle event save from EventModal component
        async handleEventSave(eventData) {
            const { title, date: eventDateStr, time: timeStr, alarmEnabled, alarmMinutesBefore, alarmType } = eventData;

            // Build schedule line: "H:MM Title" (no alarm data in diary)
            const scheduleLine = `${timeStr} ${title}`;

            try {
                // Load the target diary
                let content = await invoke('load_diary', { date: eventDateStr });
                if (!content) {
                    content = `${eventDateStr}\n`;
                }

                // If editing, remove the old line first (handles both new and legacy format)
                if (this.editingEvent) {
                    const linePattern = new RegExp(`^${this.editingEvent.time}\\s+(?:\\[A:\\d+:[nsb]\\]\\s+)?${this.editingEvent.title.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`);
                    content = content.split('\n').filter(line => !linePattern.test(line.trim())).join('\n');
                    // Remove old alarm from localStorage
                    const oldAlarmKey = `${this.date}:${this.editingEvent.time}:${this.editingEvent.title}`;
                    const alarmData = JSON.parse(localStorage.getItem('eventAlarms') || '{}');
                    delete alarmData[oldAlarmKey];
                    localStorage.setItem('eventAlarms', JSON.stringify(alarmData));
                }

                // Append the new schedule line at the end
                content = content.trimEnd() + '\n' + scheduleLine;

                // Save the diary
                await invoke('save_diary', { date: eventDateStr, content });

                // Save alarm settings to localStorage
                const alarmKey = `${eventDateStr}:${timeStr}:${title}`;
                const alarmData = JSON.parse(localStorage.getItem('eventAlarms') || '{}');
                if (alarmEnabled) {
                    alarmData[alarmKey] = {
                        enabled: true,
                        minutesBefore: alarmMinutesBefore,
                        type: alarmType
                    };
                } else {
                    delete alarmData[alarmKey];
                }
                localStorage.setItem('eventAlarms', JSON.stringify(alarmData));

                // Update view if date changed
                if (eventDateStr !== this.date) {
                    this.date = eventDateStr;
                }
                await this.loadDiaryEntry();
                await this.loadCurrentEntryTags();
                await this.loadScheduleEvents();
                await this.loadAllEntries();
                this.closeEventModal();
            } catch (error) {
                console.error('Failed to save event:', error);
            }
        },
        // Handle event delete from EventModal component
        async handleEventDelete(eventId) {
            await this.deleteScheduleEvent(eventId);
            this.closeEventModal();
        },
        async deleteScheduleEvent(eventId) {
            try {
                // Find the event to delete
                const event = this.scheduleEvents.find(e => e.id === eventId);
                if (!event) return;

                // Load diary content
                let content = await invoke('load_diary', { date: this.date });
                if (!content) return;

                // Remove the schedule line - match both new and legacy format
                // New: "H:MM Title", Legacy: "H:MM [A:min:type] Title"
                const linePattern = new RegExp(`^${event.time}\\s+(?:\\[A:\\d+:[nsb]\\]\\s+)?${event.title.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`);
                content = content.split('\n').filter(line => !linePattern.test(line.trim())).join('\n');

                // Save the diary
                await invoke('save_diary', { date: this.date, content });

                // Remove alarm from localStorage
                const alarmKey = `${this.date}:${event.time}:${event.title}`;
                const alarmData = JSON.parse(localStorage.getItem('eventAlarms') || '{}');
                delete alarmData[alarmKey];
                localStorage.setItem('eventAlarms', JSON.stringify(alarmData));

                await this.loadDiaryEntry();
                await this.loadScheduleEvents();
            } catch (error) {
                console.error('Failed to delete event:', error);
            }
        },
        // Alarm scheduler methods
        async requestNotificationPermission() {
            try {
                const { isPermissionGranted, requestPermission } = await import('@tauri-apps/plugin-notification');
                let permitted = await isPermissionGranted();
                if (!permitted) {
                    const permission = await requestPermission();
                    permitted = permission === 'granted';
                }
                return permitted;
            } catch (error) {
                console.error('Failed to request notification permission:', error);
                return false;
            }
        },
        startAlarmScheduler() {
            // Check alarms every 30 seconds
            this.alarmCheckInterval = setInterval(() => {
                this.checkAlarms();
            }, 30000);
            // Also check immediately on start
            this.checkAlarms();
        },
        async checkAlarms() {
            const now = new Date();
            const currentHour = now.getHours();
            const currentMinute = now.getMinutes();
            const currentTimeInMinutes = currentHour * 60 + currentMinute;
            const todayStr = this.getFormattedDate(now);

            // Only check events for today
            if (this.date !== todayStr) return;

            for (const event of this.scheduleEvents) {
                if (!event.alarmEnabled) continue;

                // Parse event time
                const [eventHour, eventMinute] = event.time.split(':').map(Number);
                const eventTimeInMinutes = eventHour * 60 + eventMinute;

                // Calculate alarm trigger time
                const alarmTriggerTime = eventTimeInMinutes - event.alarmMinutesBefore;

                // Check if current time matches alarm trigger time (within 1 minute window)
                const alarmKey = `${todayStr}-${event.id}`;
                if (Math.abs(currentTimeInMinutes - alarmTriggerTime) <= 1 && !this.firedAlarms.has(alarmKey)) {
                    await this.fireAlarm(event);
                    this.firedAlarms.add(alarmKey);
                }
            }
        },
        async fireAlarm(event) {
            const alarmType = event.alarmType || 'notification';

            // Send notification if type is 'notification' or 'both'
            if (alarmType === 'notification' || alarmType === 'both') {
                try {
                    const { sendNotification } = await import('@tauri-apps/plugin-notification');
                    const minutesText = event.alarmMinutesBefore === 0 ? 'now' : `in ${event.alarmMinutesBefore} min`;
                    await sendNotification({
                        title: 'Bingo Diary Reminder',
                        body: `${event.title} - ${event.time} (${minutesText})`
                    });
                } catch (error) {
                    console.error('Failed to send notification:', error);
                }
            }

            // Play sound if type is 'sound' or 'both'
            if (alarmType === 'sound' || alarmType === 'both') {
                this.playAlarmSound();
            }
        },
        playAlarmSound() {
            try {
                // Create a simple beep sound using Web Audio API
                const audioContext = new (window.AudioContext || window.webkitAudioContext)();
                const oscillator = audioContext.createOscillator();
                const gainNode = audioContext.createGain();

                oscillator.connect(gainNode);
                gainNode.connect(audioContext.destination);

                oscillator.frequency.value = 800;
                oscillator.type = 'sine';
                gainNode.gain.value = 0.3;

                oscillator.start();
                // Beep pattern: beep-pause-beep-pause-beep
                setTimeout(() => { gainNode.gain.value = 0; }, 200);
                setTimeout(() => { gainNode.gain.value = 0.3; }, 400);
                setTimeout(() => { gainNode.gain.value = 0; }, 600);
                setTimeout(() => { gainNode.gain.value = 0.3; }, 800);
                setTimeout(() => { gainNode.gain.value = 0; oscillator.stop(); }, 1000);
            } catch (error) {
                console.error('Failed to play alarm sound:', error);
            }
        }
    },
    async mounted() {
        // Detect platform for mobile UI adaptation
        try {
            const currentPlatform = await platform();
            this.isMobile = currentPlatform === 'android' || currentPlatform === 'ios';
            // Disable zen mode on mobile by default
            if (this.isMobile) {
                this.zenMode = false;
            }
        } catch (e) {
            console.log('Platform detection failed, assuming desktop');
            this.isMobile = false;
        }

        // Check password on startup
        await this.checkPassword();
        await this.loadConfig();

        window.addEventListener("keydown", this.handleGlobalShortcut);
        window.addEventListener("click", this.closeMenuDropdown);

        // Register touch events for mobile drawer gesture
        document.addEventListener('touchstart', this.handleTouchStart, { passive: true });
        document.addEventListener('touchend', this.handleTouchEnd, { passive: true });

        // Listen to window resize to save size
        const { getCurrentWindow } = await import('@tauri-apps/api/window');
        const win = getCurrentWindow();
        this.resizeUnlisten = await win.onResized(async () => {
            // Debounce: only save after resize stops
            clearTimeout(this.resizeTimer);
            this.resizeTimer = setTimeout(() => this.saveWindowSize(), 500);
        });

        await this.loadTags();
        await this.loadDiaryEntry();
        await this.loadCurrentEntryTags();
        await this.loadAllEntriesWithTags();
        await this.loadSyncStatus();

        // Listen for sync events
        await listen('sync-started', () => {
            this.syncStatus.syncing = true;
        });
        await listen('sync-completed', () => {
            this.syncStatus.syncing = false;
            this.loadSyncStatus();
            this.loadAllEntriesWithTags();
        });

        // Listen to menu events
        await listen('menu-export-json', async () => {
            try {
                await invoke('export_json');
                console.log('JSON exported successfully');
            } catch (error) {
                console.error('Export JSON failed:', error);
                alert('Failed to export JSON: ' + error);
            }
        });

        await listen('menu-export-pdf', async () => {
            try {
                await this.exportToPDF();
            } catch (error) {
                console.error('Export PDF failed:', error);
                alert('Failed to export PDF: ' + error);
            }
        });

        await listen('menu-export-markdown', async () => {
            try {
                await this.exportToMarkdown();
            } catch (error) {
                console.error('Export Markdown failed:', error);
                alert('Failed to export Markdown: ' + error);
            }
        });

        await listen('menu-load-json', async () => {
            try {
                const result = await invoke('import_json');
                console.log('JSON imported:', result);
                alert(result);
                await this.loadAllEntries();
                await this.loadDiaryEntry();
            } catch (error) {
                console.error('Import JSON failed:', error);
                alert('Failed to import JSON: ' + error);
            }
        });

        await listen('menu-open-settings', () => {
            this.openSettings();
        });

        // Initialize alarm scheduler
        await this.requestNotificationPermission();
        this.startAlarmScheduler();
    },
    beforeUnmount() {
        window.removeEventListener("keydown", this.handleGlobalShortcut);
        window.removeEventListener("click", this.closeMenuDropdown);
        document.removeEventListener('touchstart', this.handleTouchStart);
        document.removeEventListener('touchend', this.handleTouchEnd);
        if (this.resizeUnlisten) this.resizeUnlisten();
        clearTimeout(this.resizeTimer);
        // Clear alarm scheduler
        if (this.alarmCheckInterval) {
            clearInterval(this.alarmCheckInterval);
        }
    }
}
</script>


<style lang="scss">
@use "../css/index.scss";
</style>