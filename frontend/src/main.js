import { createApp } from "vue";
import App from "./App.vue";
import InstantSearch from "vue-instantsearch/vue3/es";
import VueObserveVisibility from 'vue3-observe-visibility'
import "instantsearch.css/themes/algolia-min.css";

createApp(App).use(InstantSearch).use(VueObserveVisibility).mount("#app");
