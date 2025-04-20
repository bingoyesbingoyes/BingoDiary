<template>

    <!-- get color from image: https://www.lzltool.cn/Toolkit/ExtractPngColorPalette -->
    <!-- :style="{ backgroundColor: darkTheme ? 'rgb(1, 22, 39)' : '#CAE9E8' }" -->
    <div
        class="main-wrapper"
        :style="{ backgroundColor: darkTheme ? 'rgb(150, 179, 209)' : 'rgb(150, 179, 209)' }"
    >
        <div class="left-wrapper">
            <q-input
                v-model="searchText"
                placeholder="Search."
                outlined
                rounded
                dense
                :dark="darkTheme"
                @input="filterEntries"
                class="search-input"
            />
            <!-- Calender -->
            <div v-if="!searchText" class="date-picker-container">
                <VDatePicker
                    borderless
                    transparent
                    :is-dark="darkTheme"
                    v-model="date"
                    color="orange"
                    class="date-picker"
                    @update:model-value="onDateChange"
                />
            </div>
            <!-- Search bar -->
            <div v-else class="search-results">
                <div
                    v-for="entry in filteredEntries"
                    :key="entry.date"
                    class="entry-preview"
                    @click="selectEntry(entry)"
                    :style="{ color: darkTheme ? '#fff' : '#000' }"
                >
                    <div class="entry-preview-date">{{ entry.date }}</div>
                    <p>{{ entry.content }}</p>
                </div>
            </div>
            <q-toggle
                v-model="darkTheme"
                color="black"
                size="xl"
                unchecked-icon="light_mode"
                checked-icon="dark_mode"
                @change="toggledarkTheme"
                class="toggleThemeBtn"
            />
        </div>

        <div class="right-wrapper">
            <!-- Markdown Editor -->
            <MdEditor
                v-model="text"
                :toolbars="toolbars"
                :language="'en-US'"
                :theme="darkTheme ? 'dark' : 'light'"
                :style="{ '--md-color': darkTheme ? '#fff' : '#000' }"
                @onUploadImg="onUploadImg"
            />
        </div>
    </div>
</template>

<script>
export default {
    data() {
        const today = this.getFormattedDate(new Date());
        return {
            searchText: "",
            date: today,
            text: `${today}\n`,
            entries: [],
            filteredEntries: [],
            darkTheme: true, // false -> light, true -> true
            toolbars: [
                // 'image',
                0,
            ],
        };
    },
    watch: {
        searchText(newVal) {
            this.filterEntries();
        },
    },
    methods: {
        getFormattedDate(date) {
            return new Date(date)
                .toLocaleDateString("zh-CN", {
                    timeZone: "Asia/Shanghai",
                    year: "numeric",
                    month: "2-digit",
                    day: "2-digit",
                })
                .replace(/\//g, "-");
        },
        onDateChange(newDate) {
            this.date = this.getFormattedDate(newDate);
            this.loadDiaryEntry();
        },
        loadDiaryEntry() {
            const entry = window.electronAPI.loadDiary(this.date);
            this.text = entry || `${this.date}\n`;
        },
        saveDiaryEntry() {
            const formattedDate = this.date;
            window.electronAPI.saveDiary(formattedDate, this.text);
        },
        handleSaveShortcut(event) {
            if (event.ctrlKey && event.key === "s") {
                event.preventDefault();
                this.saveDiaryEntry();
                this.loadAllEntries();
            }
        },
        filterEntries() {
            if (this.searchText) {
                this.filteredEntries = this.entries.filter((entry) =>
                    entry.content.includes(this.searchText)
                );
            } else {
                this.filteredEntries = [];
            }
        },
        loadAllEntries() {
            this.entries = window.electronAPI.getAllDiaries();
        },
        selectEntry(entry) {
            this.date = entry.date;
            this.text = entry.content;
        },
        toggledarkTheme() {
            this.darkTheme = !this.darkTheme;
        },
        async onUploadImg(files, callback) {
  try {
    const res = await Promise.all(
      files.map(async (file) => {
        const buffer = await file.arrayBuffer();
        const imageUrl = await window.electronAPI.saveImage({
          originalname: file.name,
          buffer: buffer
        });
        // 直接使用返回的协议路径
        return `${imageUrl}`;
      })
    );
    callback(res);
  } catch (error) {
    console.error('上传失败:', error);
    callback([]);
  }
}
    },
    mounted() {
        window.addEventListener("keydown", this.handleSaveShortcut);
        this.loadDiaryEntry();
        this.loadAllEntries();
    },
    beforeUnmount() {
        window.removeEventListener("keydown", this.handleSaveShortcut);
    },
};
</script>

<style lang="scss">
.main-wrapper {
    display: flex;
    width: 100%;
    height: 100vh;
    // background color is defined in <div class=main-wrapper>
    // background: rgb(1, 22, 39);
    background: url("../assets/images/lyq.png") 0% 50%  no-repeat;
    background-size: contain;
    // background color 在最上面main-wrapper
    // background: url("../assets/images/xsl.png") center center no-repeat;
    // background-size: cover;
    .left-wrapper {
        flex: 1;
        display: flex;
        flex-direction: column;
        position: relative;
        padding: 2em;
        .search-input {
            width: 100%;
            margin-top: 10%;
            margin-bottom: 10%;
            position: relative;
        }
        .date-picker-container {
            width: 100%;
            .vc-title,
            button {
                background: transparent;
            }
            .vc-dark {
                span,
                button {
                    color: #fff;
                    :hover {
                        color: #6366f1;
                    }
                }
            }
        }
        .toggleThemeBtn {
            display: flex;
            justify-content: center;
            align-items: center;
            position: absolute;
            width: 4.5em;
            bottom: 1em;
            left: 2em;
        }
        .search-results {
            display: flex;
            flex-direction: column;
            gap: 1em;
        }
        .entry-preview {
            padding: 1em;
            background: rgba(255, 255, 255, 0.25);
            box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
            backdrop-filter: blur(4px);
            border-radius: 10px;
            border: 1px solid rgba(255, 255, 255, 0.18);
            cursor: pointer;
            .entry-preview-date {
                font-weight: bold;
            }
        }
    }

    .right-wrapper {
        flex: 3;
        display: flex;
        flex-direction: column;
        padding: 2em;
        overflow: hidden;
    }

    .md-editor {
        flex-grow: 1;
        height: 100%;
        // --md-color: #fff;
        // font-family: 'Monospace';
        background: rgba(255, 255, 255, 0.1);
        box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
        backdrop-filter: blur(4px);
        border-radius: 10px;
        border: 1px solid rgba(255, 255, 255, 0.18);
    }
}
</style>
