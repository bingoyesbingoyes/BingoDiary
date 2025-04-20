import VCalendar from "v-calendar";
import "v-calendar/style.css";

import { MdEditor } from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import "md-editor-v3/lib/preview.css";

import { ExportPDF } from '@vavt/v3-extension';
import '@vavt/v3-extension/lib/asset/ExportPDF.css';


export default ({ app }) => {
    app.use(VCalendar);
    app.component("MdEditor", MdEditor);
    app.component("ExportPDF", ExportPDF);
};
