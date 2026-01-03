<template>
    <div class="tag-stats" :class="{ 'dark-mode': darkTheme }">
        <div class="stats-header">
            <h3>{{ t('tagManager.statistics') }}</h3>
        </div>

        <div v-if="stats.length === 0" class="no-stats">
            <p>{{ t('tagManager.noStats') }}</p>
        </div>

        <div v-else class="stats-list">
            <div v-for="stat in stats" :key="stat.tag.id" class="stat-item">
                <div class="stat-tag">
                    <span class="tag-color" :style="{ background: stat.tag.color }"></span>
                    <span class="tag-name">{{ stat.tag.name }}</span>
                </div>
                <div class="stat-bar-container">
                    <div
                        class="stat-bar"
                        :style="{
                            width: getBarWidth(stat.count) + '%',
                            background: stat.tag.color
                        }"
                    ></div>
                </div>
                <div class="stat-info">
                    <span class="stat-count">{{ stat.count }}</span>
                    <span class="stat-label">{{ t('tagManager.diary') }}</span>
                </div>
                <div class="stat-last-used" v-if="stat.lastUsed">
                    <span class="last-used-label">{{ t('tagManager.last') }}</span>
                    <span class="last-used-date">{{ formatDate(stat.lastUsed) }}</span>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    name: 'TagStats',
    props: {
        stats: {
            type: Array,
            default: () => []
        },
        darkTheme: {
            type: Boolean,
            default: false
        },
        t: {
            type: Function,
            required: true
        }
    },
    computed: {
        maxCount() {
            if (this.stats.length === 0) return 1;
            return Math.max(...this.stats.map(s => s.count), 1);
        }
    },
    methods: {
        getBarWidth(count) {
            return (count / this.maxCount) * 100;
        },
        formatDate(dateStr) {
            if (!dateStr) return '';
            const date = new Date(dateStr);
            const today = new Date();
            const yesterday = new Date(today);
            yesterday.setDate(yesterday.getDate() - 1);

            const dateOnly = dateStr.split('T')[0];
            const todayStr = today.toISOString().split('T')[0];
            const yesterdayStr = yesterday.toISOString().split('T')[0];

            if (dateOnly === todayStr) return this.t('common.today');
            if (dateOnly === yesterdayStr) return this.t('common.yesterday');

            // Format as MM/DD
            const parts = dateOnly.split('-');
            return `${parts[1]}/${parts[2]}`;
        }
    }
}
</script>

<style lang="scss" scoped>
.tag-stats {
    .stats-header {
        margin-bottom: 1rem;

        h3 {
            font-size: 0.85rem;
            color: #666;
            margin: 0;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
    }

    .no-stats {
        text-align: center;
        color: #999;
        padding: 1.5rem;
        font-size: 0.9rem;
    }

    .stats-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .stat-item {
        display: grid;
        grid-template-columns: 100px 1fr auto auto;
        gap: 12px;
        align-items: center;
        padding: 10px 12px;
        background: rgba(0, 0, 0, 0.03);
        border-radius: 10px;
        transition: background 0.15s;

        &:hover {
            background: rgba(0, 0, 0, 0.06);
        }
    }

    .stat-tag {
        display: flex;
        align-items: center;
        gap: 8px;
        overflow: hidden;

        .tag-color {
            width: 10px;
            height: 10px;
            border-radius: 50%;
            flex-shrink: 0;
        }

        .tag-name {
            font-size: 0.85rem;
            font-weight: 500;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }
    }

    .stat-bar-container {
        height: 8px;
        background: rgba(0, 0, 0, 0.08);
        border-radius: 4px;
        overflow: hidden;
    }

    .stat-bar {
        height: 100%;
        border-radius: 4px;
        transition: width 0.3s ease;
        min-width: 4px;
    }

    .stat-info {
        display: flex;
        align-items: baseline;
        gap: 4px;
        min-width: 60px;
        justify-content: flex-end;

        .stat-count {
            font-size: 0.95rem;
            font-weight: 600;
            color: #333;
        }

        .stat-label {
            font-size: 0.75rem;
            color: #888;
        }
    }

    .stat-last-used {
        display: flex;
        flex-direction: column;
        align-items: flex-end;
        min-width: 60px;

        .last-used-label {
            font-size: 0.65rem;
            color: #aaa;
            text-transform: uppercase;
        }

        .last-used-date {
            font-size: 0.8rem;
            color: #666;
        }
    }

    // Dark mode
    &.dark-mode {
        .stats-header h3 {
            color: #88c0d0;
        }

        .no-stats {
            color: #888;
        }

        .stat-item {
            background: rgba(255, 255, 255, 0.05);

            &:hover {
                background: rgba(255, 255, 255, 0.08);
            }
        }

        .stat-tag .tag-name {
            color: #d8dee9;
        }

        .stat-bar-container {
            background: rgba(255, 255, 255, 0.1);
        }

        .stat-info {
            .stat-count {
                color: #d8dee9;
            }

            .stat-label {
                color: #888;
            }
        }

        .stat-last-used {
            .last-used-label {
                color: #666;
            }

            .last-used-date {
                color: #999;
            }
        }
    }
}
</style>
