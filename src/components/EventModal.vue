<template>
    <div v-if="show" class="event-modal-overlay" @click.self="$emit('close')">
        <div class="event-modal" :class="{ 'dark-mode': darkTheme }">
            <div class="event-modal-header">
                <h2>{{ editingEvent ? t('event.editEvent') : t('event.newEvent') }}</h2>
                <button class="modal-close-btn" @click="$emit('close')">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                    </svg>
                </button>
            </div>
            <div class="event-modal-body">
                <!-- Date & Time Picker - Scrollable drums -->
                <div class="datetime-picker-row">
                    <!-- Date drums: Year / Month / Day -->
                    <div class="date-drums">
                        <div class="drum-column"
                             :class="{ 'drum-active': activeDrum === 'year' }"
                             @wheel.prevent="handleScroll($event, 'year')"
                             @touchstart.prevent="drumTouchStart($event, 'year')"
                             @touchmove.prevent="drumTouchMove($event, 'year')"
                             @touchend="drumTouchEnd">
                            <div class="drum-value small">{{ eventYear }}</div>
                        </div>
                        <span class="date-separator">/</span>
                        <div class="drum-column"
                             :class="{ 'drum-active': activeDrum === 'month' }"
                             @wheel.prevent="handleScroll($event, 'month')"
                             @touchstart.prevent="drumTouchStart($event, 'month')"
                             @touchmove.prevent="drumTouchMove($event, 'month')"
                             @touchend="drumTouchEnd">
                            <div class="drum-value small">{{ eventMonth.toString().padStart(2, '0') }}</div>
                        </div>
                        <span class="date-separator">/</span>
                        <div class="drum-column"
                             :class="{ 'drum-active': activeDrum === 'day' }"
                             @wheel.prevent="handleScroll($event, 'day')"
                             @touchstart.prevent="drumTouchStart($event, 'day')"
                             @touchmove.prevent="drumTouchMove($event, 'day')"
                             @touchend="drumTouchEnd">
                            <div class="drum-value small">{{ eventDay.toString().padStart(2, '0') }}</div>
                        </div>
                    </div>

                    <!-- Time drums: Hour : Minute -->
                    <div class="time-drums">
                        <div class="drum-column"
                             :class="{ 'drum-active': activeDrum === 'hour' }"
                             @wheel.prevent="handleScroll($event, 'hour')"
                             @touchstart.prevent="drumTouchStart($event, 'hour')"
                             @touchmove.prevent="drumTouchMove($event, 'hour')"
                             @touchend="drumTouchEnd">
                            <div class="drum-value">{{ eventHour.toString().padStart(2, '0') }}</div>
                        </div>
                        <span class="time-separator">:</span>
                        <div class="drum-column"
                             :class="{ 'drum-active': activeDrum === 'minute' }"
                             @wheel.prevent="handleScroll($event, 'minute')"
                             @touchstart.prevent="drumTouchStart($event, 'minute')"
                             @touchmove.prevent="drumTouchMove($event, 'minute')"
                             @touchend="drumTouchEnd">
                            <div class="drum-value">{{ eventMinute.toString().padStart(2, '0') }}</div>
                        </div>
                    </div>
                </div>

                <!-- Alarm Settings -->
                <div class="alarm-settings">
                    <div class="alarm-toggle-row">
                        <label class="alarm-toggle">
                            <input type="checkbox" v-model="alarmEnabled" />
                            <span class="toggle-slider"></span>
                        </label>
                        <span class="alarm-label">{{ t('event.alarm') }}</span>
                    </div>
                    <div v-if="alarmEnabled" class="alarm-options">
                        <div class="alarm-option-row">
                            <input
                                type="number"
                                v-model.number="alarmMinutesBefore"
                                min="0"
                                max="1440"
                                class="alarm-minutes-input"
                            />
                            <span class="alarm-unit">{{ t('event.minBefore') }}</span>
                        </div>
                        <div class="alarm-type-row">
                            <label class="alarm-type-option">
                                <input type="radio" v-model="alarmType" value="notification" />
                                <span>{{ t('event.notification') }}</span>
                            </label>
                            <label class="alarm-type-option">
                                <input type="radio" v-model="alarmType" value="sound" />
                                <span>{{ t('event.sound') }}</span>
                            </label>
                            <label class="alarm-type-option">
                                <input type="radio" v-model="alarmType" value="both" />
                                <span>{{ t('event.both') }}</span>
                            </label>
                        </div>
                    </div>
                </div>

                <!-- Quick-fill tags (append to title) -->
                <div class="quickfill-section">
                    <div class="quickfill-header">
                        <button class="edit-cats-btn" @click="editingCategories = !editingCategories">
                            {{ editingCategories ? t('common.done') : t('common.edit') }}
                        </button>
                    </div>
                    <div class="quickfill-chips">
                        <template v-if="!editingCategories">
                            <button
                                v-for="cat in categories"
                                :key="cat.id"
                                class="quickfill-chip"
                                @click="appendToTitle(cat.name)"
                            >
                                {{ cat.name }}
                            </button>
                        </template>
                        <template v-else>
                            <div v-for="(cat, index) in categories" :key="cat.id" class="category-edit-item">
                                <input
                                    :value="cat.name"
                                    @input="updateCategoryName(index, $event.target.value)"
                                    type="text"
                                    class="category-name-input"
                                />
                                <button class="delete-cat-btn" @click="deleteCategory(index)">×</button>
                            </div>
                            <div class="add-category-row">
                                <input
                                    v-model="newCategoryName"
                                    type="text"
                                    :placeholder="t('event.newCategory')"
                                    class="category-name-input"
                                    @keyup.enter="addCategory"
                                />
                                <button class="add-cat-btn" @click="addCategory">+</button>
                            </div>
                        </template>
                    </div>
                </div>

                <!-- Title input at bottom -->
                <div class="inline-field bottom-input">
                    <input
                        ref="eventTitleInput"
                        v-model="eventTitle"
                        type="text"
                        :placeholder="t('event.event')"
                        class="inline-input title-input"
                    />
                </div>
            </div>
            <div class="event-modal-footer">
                <button v-if="editingEvent" class="delete-btn" @click="handleDelete">
                    {{ t('common.delete') }}
                </button>
                <button class="save-btn" @click="handleSave">
                    {{ t('common.save') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue';

const props = defineProps({
    show: Boolean,
    darkTheme: Boolean,
    editingEvent: Object,
    initialDate: String,
    categories: Array,
    t: { type: Function, required: true }
});

const emit = defineEmits(['close', 'save', 'delete', 'update-categories']);

// Event data
const eventTitle = ref('');
const eventYear = ref(new Date().getFullYear());
const eventMonth = ref(new Date().getMonth() + 1);
const eventDay = ref(new Date().getDate());
const eventHour = ref(9);
const eventMinute = ref(0);

// Alarm settings
const alarmEnabled = ref(false);
const alarmMinutesBefore = ref(10);
const alarmType = ref('notification');

// Drum picker state
const activeDrum = ref(null);
const drumTouchStartY = ref(0);
const drumTouchLastY = ref(0);

// Category editing
const editingCategories = ref(false);
const newCategoryName = ref('');

// Reference to title input
const eventTitleInput = ref(null);

// Watch for show changes to initialize data
watch(() => props.show, (newVal) => {
    if (newVal) {
        initializeModal();
    }
});

// Watch for editing event changes
watch(() => props.editingEvent, () => {
    if (props.show) {
        initializeModal();
    }
});

function initializeModal() {
    if (props.editingEvent) {
        // Editing existing event
        eventTitle.value = props.editingEvent.title;
        const [h, m] = props.editingEvent.time.split(':').map(Number);
        eventHour.value = h;
        eventMinute.value = m;
        alarmEnabled.value = props.editingEvent.alarmEnabled || false;
        alarmMinutesBefore.value = props.editingEvent.alarmMinutesBefore || 10;
        alarmType.value = props.editingEvent.alarmType || 'notification';
    } else {
        // New event - use current time
        const now = new Date();
        eventTitle.value = '';
        eventHour.value = now.getHours();
        eventMinute.value = now.getMinutes();
        alarmEnabled.value = false;
        alarmMinutesBefore.value = 10;
        alarmType.value = 'notification';
    }

    // Set date from initialDate prop
    if (props.initialDate) {
        const dateObj = new Date(props.initialDate + 'T00:00:00');
        eventYear.value = dateObj.getFullYear();
        eventMonth.value = dateObj.getMonth() + 1;
        eventDay.value = dateObj.getDate();
    }

    editingCategories.value = false;
    newCategoryName.value = '';
}

// Drum picker adjustment methods
function adjustYear(delta) {
    const currentYear = new Date().getFullYear();
    eventYear.value = Math.max(currentYear - 5, Math.min(currentYear + 5, eventYear.value + delta));
}

function adjustMonth(delta) {
    let newMonth = eventMonth.value + delta;
    if (newMonth > 12) newMonth = 1;
    if (newMonth < 1) newMonth = 12;
    eventMonth.value = newMonth;
    // Clamp day if needed
    const maxDay = new Date(eventYear.value, eventMonth.value, 0).getDate();
    if (eventDay.value > maxDay) eventDay.value = maxDay;
}

function adjustDay(delta) {
    const maxDay = new Date(eventYear.value, eventMonth.value, 0).getDate();
    let newDay = eventDay.value + delta;
    if (newDay > maxDay) newDay = 1;
    if (newDay < 1) newDay = maxDay;
    eventDay.value = newDay;
}

function adjustHour(delta) {
    eventHour.value = (eventHour.value + delta + 24) % 24;
}

function adjustMinute(delta) {
    eventMinute.value = (eventMinute.value + delta + 60) % 60;
}

function handleScroll(event, type) {
    const delta = event.deltaY > 0 ? -1 : 1;
    switch (type) {
        case 'year': adjustYear(delta); break;
        case 'month': adjustMonth(delta); break;
        case 'day': adjustDay(delta); break;
        case 'hour': adjustHour(delta); break;
        case 'minute': adjustMinute(delta); break;
    }
}

// Touch handlers for drum picker (mobile)
function drumTouchStart(event, type) {
    drumTouchStartY.value = event.touches[0].clientY;
    drumTouchLastY.value = event.touches[0].clientY;
    activeDrum.value = type;
}

function drumTouchMove(event, type) {
    const currentY = event.touches[0].clientY;
    const deltaY = drumTouchLastY.value - currentY;
    const threshold = 20; // pixels to trigger a change

    if (Math.abs(deltaY) >= threshold) {
        const delta = deltaY > 0 ? 1 : -1;
        switch (type) {
            case 'year': adjustYear(delta); break;
            case 'month': adjustMonth(delta); break;
            case 'day': adjustDay(delta); break;
            case 'hour': adjustHour(delta); break;
            case 'minute': adjustMinute(delta); break;
        }
        drumTouchLastY.value = currentY;
    }
}

function drumTouchEnd() {
    activeDrum.value = null;
}

// Title helpers
function appendToTitle(text) {
    if (eventTitle.value) {
        eventTitle.value += ' ' + text;
    } else {
        eventTitle.value = text;
    }
}

// Category management
function updateCategoryName(index, name) {
    const updatedCategories = [...props.categories];
    updatedCategories[index] = { ...updatedCategories[index], name };
    emit('update-categories', updatedCategories);
}

function deleteCategory(index) {
    if (props.categories.length > 1) {
        const updatedCategories = props.categories.filter((_, i) => i !== index);
        emit('update-categories', updatedCategories);
    }
}

function addCategory() {
    if (newCategoryName.value.trim()) {
        const newId = `cat-${Date.now()}`;
        const updatedCategories = [...props.categories, {
            id: newId,
            name: newCategoryName.value.trim(),
            color: '#4A8B8B'
        }];
        emit('update-categories', updatedCategories);
        newCategoryName.value = '';
    }
}

// Save/Delete handlers
function handleSave() {
    if (!eventTitle.value.trim()) {
        return;
    }

    const eventData = {
        title: eventTitle.value.trim(),
        date: `${eventYear.value}-${eventMonth.value.toString().padStart(2, '0')}-${eventDay.value.toString().padStart(2, '0')}`,
        time: `${eventHour.value}:${eventMinute.value.toString().padStart(2, '0')}`,
        alarmEnabled: alarmEnabled.value,
        alarmMinutesBefore: alarmMinutesBefore.value,
        alarmType: alarmType.value
    };

    emit('save', eventData);
}

function handleDelete() {
    if (props.editingEvent) {
        emit('delete', props.editingEvent.id);
    }
}
</script>
