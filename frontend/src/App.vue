<template>
  <div class="container">
    <ais-instant-search :search-client="searchClient" :index-name="indexNameInput">
      <header class="header">
        <h1 class="header-title">MeiliFileFinder</h1>
        <div class="search-panel__filters"></div>
        <app-debounced-search-box :delay="5" class="ais-SearchBox-input" />
        <div>
        <ais-sort-by :items="[
          { value: 'filesystem_index', label: 'Relevant' },
          {
            value: 'filesystem_index:modified_date:desc',
            label: 'Newest',
          },
          {
            value: 'filesystem_index:modified_date:asc',
            label: 'Oldest',
          },
        ]" />
        </div>

        <button class="api-key-button" @click="showDialog = true">Connection Settings</button>
      </header>

      <div class="search-panel__results">
        <ais-infinite-hits>
          <template v-slot:item="{ item }">
            <div>
              <div class="hit-name">
                <ais-snippet :hit="item" attribute="path" />
              </div>
            </div>
          </template>
          <template v-slot:loadMore="{ isLastPage, refineNext }">
            <button
              class="ais-InfiniteHits-loadMore"
              :disabled="isLastPage"
              v-observe-visibility="refineNext"
              @click="refineNext"
            >
              Show more results
            </button>
          </template>
        </ais-infinite-hits>
        <ais-configure :analytics="false" :attributesToSnippet="['path:50']" hitsPerPage="128" snippetEllipsisText="…" />
      </div>
      <!-- <ais-pagination /> -->
    </ais-instant-search>
  </div>

  <div v-if="showDialog" class="dialog-overlay">
    <div class="dialog-box">
      <h2>Connection Settings</h2>
      <p>
        <span>Meilisearch Master Key</span>
        <input type="text" v-model="masterKeyInput" placeholder="Enter API Key" class="api-key-input" />
      </p>
      <p>
        <span>Index Name</span>
        <input type="text" v-model="indexNameInput" placeholder="Enter Index Name" class="api-key-input" />
      </p>
      <button @click="saveMasterKey" class="confirm-button">Confirm</button>
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
    const storedMasterKey = localStorage.getItem("meilisearchMasterKey") || "hello_world123456";
    const storedIndexName = localStorage.getItem("meilisearchIndexName") || "filesystem_index";
    //let base_url = `${window.location.origin}${window.location.pathname}`;
    //let url = base_url + (base_url.endsWith('/') ? 'meilisearch' : '/meilisearch');
    let url = "https://wg.hafuhafuhako.uk:16031/meilisearch"

    return {
      masterKeyInput: storedMasterKey,
      indexNameInput: storedIndexName,
      showDialog: false,
      searchClient: instantMeiliSearch(
        url,
        storedMasterKey,
        {
          finitePagination: true,
        }
      ).searchClient,
    };
  },
  methods: {
    visibilityChanged(isVisible) {
      console.log("Visibility changed: ", isVisible);
    },
    saveMasterKey() {
      localStorage.setItem("meilisearchMasterKey", this.masterKeyInput);
      localStorage.setItem("meilisearchIndexName", this.indexNameInput);
      let base_url = `${window.location.origin}${window.location.pathname}`;
      let url = base_url + (base_url.endsWith('/') ? 'meilisearch' : '/meilisearch');
      this.searchClient = instantMeiliSearch(
        url,
        this.masterKeyInput,
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

.ais-InfiniteHits-list {
  padding: 0rem .65rem 0rem .65rem !important;
  display: block !important;
}

.ais-InfiniteHits-item {
  width: auto !important;
  padding: 0px !important;
  box-shadow: none !important;
  margin-top: 0px !important;
  margin-left: 1em !important;
  margin-bottom: 0px !important;
  margin-right: 0px !important;
  padding-left: 2em !important;
  padding-top: 0.3em !important;
  padding-bottom: 0.3em !important;
  border-color: #eee !important;
  border-left: 0;
  border-right: 0;
  /* margin-bottom: 1em; */
  /*width: calc(50% - 1rem);*/
}

.ais-InfiniteHits-item img {
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

.hit-info {
  font-size: 90%;
}

.hit-name {
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;700&display=swap');
  font-family: 'Noto Sans SC', sans-serif;
  line-height: 16px;
  font-size: 1em;
}

.hit-description {
  font-size: 90%;
  margin-bottom: 0.5em;
  color: grey;
}

.header-subtitle {
  font-size: 1.2rem;
}


.ais-InstantSearch {
  /* max-width: 960px; */
  overflow: hidden;
  margin: 0;
}

.search-panel__filters {
  float: left;
  width: 20px;
}


.ais-SearchBox {
  margin-bottom: 2rem;
}

.ais-SearchBox-input {
  width: auto !important;
  padding: 0 .5rem 0 1.7rem !important;
  border-radius: 0 !important;
  border: 1px solid #ccc;
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;700&display=swap');
  font-family: 'Noto Sans SC', sans-serif;
  line-height: 18px;
  font-size: 1em;
}

input.ais-SearchBox-input {
  width: 100% !important;
  border: 0;
  padding: .3rem 0 .3rem .3rem !important;
}

.ais-SortBy-select {
  border-radius: 3px !important;
  padding-left: .7rem !important;
}

.ais-SortBy {
  display: flex;
  margin: 0 1rem;
}

.ais-Pagination-item--selected .ais-Pagination-link {
  color: #fff !important;
  background-color: #369988 !important;
  border-color: #369988 !important;
}

.ais-Pagination-link {
  border-radius: 3px !important;
  color: #369988 !important;
}

.ais-Pagination {
  margin: 2rem auto;
  text-align: center;
}

.ais-SearchBox-form {
  /* margin: 10px 10px 30px 10px; */
  flex: 1;
}

.header {
  display: flex;
  align-items: center;
  min-height: 50px;
  padding: 0 1rem;
  background-image: linear-gradient(to right, #4dba87, #2f9088);
  color: #fff;
}

.header-title {
  font-size: 1.2rem;
  @import url('https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@400;700&display=swap');
  font-family: 'Noto Sans SC', sans-serif;
  line-height: 16px;
  font-weight: normal;
}

.api-key-button {
  background: #ffffff;
  color: #2f9088;
  border: none;
  border-radius: 3px;
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

.search-panel__results {
  flex-direction: column;
  align-items: center;
  justify-content: center;
  margin-top: 2rem;
}

.ais-InfiniteHits-loadMore {
  margin-top: 0 !important;
  width: 100%;
  background-color: #fff !important;
  color: #2f9088 !important;
}

.ais-InfiniteHits-loadMore[disabled] {
  display: none;
}
</style>
