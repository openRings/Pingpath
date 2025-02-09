import { createApp } from "vue";
import { createRouter, createWebHistory, RouterView } from "vue-router";
import Index from "./pages/index.vue";
import Profile from "./pages/profile.vue";
import Pricing from "./pages/pricing.vue";

import "./main.css";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: Index },
    { path: "/profile", component: Profile },
    { path: "/pricing", component: Pricing },
  ],
});

createApp(RouterView).use(router).mount("body");
