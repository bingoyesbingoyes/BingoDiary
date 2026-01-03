<template>
    <div class="lock-screen" :class="{ 'dark-mode': darkTheme }">
        <div class="lock-content">
            <div class="lock-icon">🔒</div>
            <h1>{{ t('lock.title') }}</h1>
            <input
                type="password"
                v-model="password"
                @keyup.enter="$emit('unlock', password)"
                :placeholder="t('lock.enterPassword')"
                class="lock-input"
                autofocus
            />
            <button @click="$emit('unlock', password)" class="lock-btn">{{ t('lock.unlock') }}</button>
            <p v-if="error" class="lock-error">{{ error }}</p>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';

defineProps({
    darkTheme: Boolean,
    error: String,
    t: { type: Function, required: true }
});

defineEmits(['unlock']);

const password = ref('');
</script>
