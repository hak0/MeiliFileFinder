<template>
  <header class="header">
    <h1 class="header-title">MeiliFileFinder</h1>
    <!-- <p class="header-subtitle">Search in Steam video games 🎮</p> -->
  </header>
  <p class="disclaimer">
    <!-- This is not the official Steam dataset but only for demo purpose. Enjoy -->
    <!-- searching with MeiliSearch! -->
     This is a altered search frontend from meilisearch official Vue3 demo.
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
        <!-- <h2>Is_hidden</h2> -->
        <!-- <ais-refinement-list attribute="is_hidden" /> -->
      </div>
      <div class="search-panel__results">
        <app-debounced-search-box :delay="5" class="ais-SearchBox-input" />
        <ais-hits>
          <template v-slot:item="{ item }">
            <div>
              <div class="hit-name">
                <ais-snippet :hit="item" attribute="path" />
                <!-- <ais-highlight :hit="item" attribute="name" /> -->
              </div>
              <!-- <img :src="item.image" align="left" :alt="item.image" /> -->
              <!-- <div class="hit-description"> -->
                <!-- <ais-snippet :hit="item" attribute="path" /> -->
              <!-- </div> -->
              <!-- <div class="hit-info">price: {{ item.price }}</div> -->
              <!-- <div class="hit-info">release date: {{ item.releaseDate }}</div> -->
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
</template>

<script>
import { instantMeiliSearch } from "@meilisearch/instant-meilisearch";
import AppDebouncedSearchBox from "./DebouncedSearchBox";

export default {
  components: {
    AppDebouncedSearchBox,
  },
  data() {
    return {
      searchClient: instantMeiliSearch(
        "http://localhost:3000/meilisearch",
        "hello_world123456",
        {
          finitePagination: true,
        }
      ).searchClient,
    };
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
</style>
