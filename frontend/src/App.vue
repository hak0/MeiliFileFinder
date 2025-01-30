<template>
  <header class="header">
    <h1 class="header-title">MeiliFileFinder</h1>
    <button class="api-key-button" @click="showDialog = true">Set API Key</button>
  </header>
  <p class="disclaimer">
    This is an altered search frontend from Meilisearch's official Vue3 demo.
  </p>
  <div class="container">
    <ais-instant-search
      :search-client="searchClient"
      index-name="filesystem_index"
    >
      <div class="search-panel__filters">
        <ais-sort-by
          :items="[
            { value: 'filesystem_index', label: 'Relevant' },
            {
              value: 'filesystem_index:modified_date:desc',
              label: 'Newest',
            },
            {
              value: 'filesystem_index:modified_date:asc',
              label: 'Oldest',
            },
          ]"
        />
      </div>
      <div class="search-panel__results">
        <app-debounced-search-box :delay="5" class="ais-SearchBox-input" />
        <ais-hits>
          <template v-slot:item="{ item }">
            <div>
              <div class="hit-name">
                <ais-snippet :hit="item" attribute="path" />
              </div>
            </div>
          </template>
        </ais-hits>
        <ais-configure
          :attributesToSnippet="['path:50']"
          snippetEllipsisText="…"
        />
      </div>
      <ais-pagination />
    </ais-instant-search>
  </div>

  <div v-if="showDialog" class="dialog-overlay">
    <div class="dialog-box">
      <h2>Set API Key</h2>
      <input
        type="text"
        v-model="apiKeyInput"
        placeholder="Enter API Key"
        class="api-key-input"
      />
      <button @click="saveApiKey" class="confirm-button">Confirm</button>
      <button @click="closeDialog" class="cancel-button">Cancel</button>
    </div>
  </div>
</template>

<script>
import { instantMeiliSearch } from "@meilisearch/instant-meilisearch";
import AppDebouncedSearchBox from "./DebouncedSearchBox";

export default {
  components: {
    AppDebouncedSearchBox,
  },
  data() {
    const storedApiKey = localStorage.getItem("meilisearchApiKey") || "hello_world123456";
    let base_url = `${window.location.origin}${window.location.pathname}`;
    let url = base_url + (base_url.endsWith('/') ? 'meilisearch' : '/meilisearch');
    return {
      apiKeyInput: storedApiKey,
      showDialog: false,
      searchClient: instantMeiliSearch(
        url,
        storedApiKey,
        {
          finitePagination: true,
        }
      ).searchClient,
    };
  },
  methods: {
    saveApiKey() {
      localStorage.setItem("meilisearchApiKey", this.apiKeyInput);
      let base_url = `${window.location.origin}${window.location.pathname}`;
      let url = base_url + (base_url.endsWith('/') ? 'meilisearch' : '/meilisearch');
      this.searchClient = instantMeiliSearch(
        url,
        this.apiKeyInput,
        {
          finitePagination: true,
        }
      ).searchClient;
      this.closeDialog();
    },
    closeDialog() {
      this.showDialog = false;
    },
  },
};
</script>

<style>
body,
h1 {
  margin: 0;
  padding: 0;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica,
    Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
}

.ais-Hits-list {
  display:block !important;
}

.ais-Hits-item {
  width: auto !important;
  padding: 0px !important;
  box-shadow: none !important;
  margin-top: 0px !important;
  margin-left: 1em !important;
  margin-bottom: 0px !important;
  margin-right: 0px !important;
  padding-left: 1em !important;
  padding-top: 0.2em !important;
  padding-bottom: 0.2em !important;
  border-color: #eee !important;
  /* margin-bottom: 1em; */
  /*width: calc(50% - 1rem);*/
}

.ais-Hits-item img {
  margin-right: 1em;
  width: 100%;
  height: 100%;
  margin-bottom: 0.5em;
}

.ais-Highlight-highlighted {
  background: cyan;
  font-style: normal;
}

.disclaimer {
  margin-left: 1em;
}

.hit-name {
  margin-bottom: 0.5em;
}

.hit-info {
  font-size: 90%;
}

.hit-description {
  font-size: 90%;
  margin-bottom: 0.5em;
  color: grey;
}

.header-title::after {
  content: " ▸ ";
  padding: 0 0.5rem;
}

.header-subtitle {
  font-size: 1.2rem;
}

.container {
  padding: 1rem;
}

.ais-InstantSearch {
  /* max-width: 960px; */
  overflow: hidden;
  margin: 0;
}

.search-panel__filters {
  float: left;
  width: 200px;
}

.search-panel__results {
  margin-left: 210px;
}

.ais-SearchBox {
  margin-bottom: 2rem;
}

.ais-Pagination {
  margin: 2rem auto;
  text-align: center;
}
.ais-SearchBox-form {
  margin-bottom: 20px;
}

.header {
  display: flex;
  align-items: center;
  min-height: 50px;
  padding: 0.5rem 1rem;
  background-image: linear-gradient(to right, #4dba87, #2f9088);
  color: #fff;
  margin-bottom: 1rem;
}

.header-title {
  font-size: 1.2rem;
  font-weight: normal;
  flex: 1;
}

.api-key-button {
  background: #ffffff;
  color: #2f9088;
  border: none;
  border-radius: 8px;
  padding: 0.5em 1em;
  cursor: pointer;
  font-size: 0.9rem;
}

.api-key-button:hover {
  background: #f0f0f0;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.dialog-box {
  background: #fff;
  padding: 2em;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.api-key-input {
  width: 100%;
  padding: 0.5em;
  margin-bottom: 1em;
  font-size: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.confirm-button {
  background: #4dba87;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.5em 1em;
  cursor: pointer;
}

.confirm-button:hover {
  background: #3a9f6e;
}

.cancel-button {
  background: #ccc;
  color: black;
  border: none;
  border-radius: 4px;
  padding: 0.5em 1em;
  cursor: pointer;
  margin-left: 0.5em;
}

.cancel-button:hover {
  background: #bbb;
}
</style>
