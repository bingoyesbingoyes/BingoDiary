import { createApp } from 'vue'
import App from './App.vue'
import './css/app.scss'

// Import v-calendar
import { DatePicker as VDatePicker } from 'v-calendar'
import 'v-calendar/style.css'

const app = createApp(App)

// Register VDatePicker globally
app.component('VDatePicker', VDatePicker)

app.mount('#app')
