export default defineNuxtConfig({
    compatibilityDate: '2025-05-15',
    css: ["~/assets/style.css"],
    devtools: { enabled: true },
    modules: ['@nuxt/ui', '@nuxt/eslint', "@pinia/nuxt"],
    ui: {
        colorMode: false,
    },
    appConfig: {
        API_URL: process.env.API_URL,
        API_KEY: process.env.API_KEY,
        NSFW_CATEGORIES: process.env.NSFW_CATEGORIES?.split(",").sort() || [],
        SFW_CATEGORIES: process.env.SFW_CATEGORIES?.split(",").sort() || [],
    },
    runtimeConfig: {
        API_URL: process.env.API_URL,
        API_KEY: process.env.API_KEY,
        NSFW_CATEGORIES: process.env.NSFW_CATEGORIES?.split(",").sort() || [],
        SFW_CATEGORIES: process.env.SFW_CATEGORIES?.split(",").sort() || [],
    }
});
