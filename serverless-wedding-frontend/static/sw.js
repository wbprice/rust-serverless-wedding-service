importScripts('/_nuxt/workbox.4c4f5ca6.js')

workbox.precaching.precacheAndRoute([
  {
    "url": "/_nuxt/004495bc3d3704efca4e.js",
    "revision": "a0f4cb2834b048565bb3c3c661ad55da"
  },
  {
    "url": "/_nuxt/05cdb63c4236607c5544.js",
    "revision": "9bba63ffcc7043b5d700d96a283682ac"
  },
  {
    "url": "/_nuxt/12f84eac744a4b1189aa.js",
    "revision": "9bba52f6a2a53a71e17bebc0a5b5cf97"
  },
  {
    "url": "/_nuxt/1b0e98eb5d802646634a.js",
    "revision": "61f0c7b5d38ac20ca52543844985fc53"
  },
  {
    "url": "/_nuxt/224406571a809426e45e.js",
    "revision": "b9ff360cadabb1ce2d2c519d09cf1db5"
  },
  {
    "url": "/_nuxt/aa3d71343cabe87369e8.js",
    "revision": "b429098d1b77c9ef16921d3765ff53d5"
  },
  {
    "url": "/_nuxt/b71d38d85711b5446180.js",
    "revision": "00c87f0d3e26a31724b21a719a1933c7"
  },
  {
    "url": "/_nuxt/badfff392d3060c7aa15.js",
    "revision": "0bf1eeca6550e19dfba1190b6741274d"
  },
  {
    "url": "/_nuxt/bbaa6332a187b8876ee7.js",
    "revision": "896702cc119acb873b965ad301fec913"
  },
  {
    "url": "/_nuxt/c08073998d1cd7f82ec7.js",
    "revision": "b407a3cecc69f93682126b98c4f60313"
  },
  {
    "url": "/_nuxt/ccb89143e4fe264e5069.js",
    "revision": "06dd7d2ddafa125056d0982367ea4e79"
  }
], {
  "cacheId": "blinging.love",
  "directoryIndex": "/",
  "cleanUrls": false
})

workbox.clientsClaim()
workbox.skipWaiting()

workbox.routing.registerRoute(new RegExp('/_nuxt/.*'), workbox.strategies.cacheFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('/.*'), workbox.strategies.networkFirst({}), 'GET')

workbox.routing.registerRoute(new RegExp('https://cdnjs.cloudflare.com/ajax/libs/skeleton/.*'), workbox.strategies.cacheFirst({"cacheableResponse":{"statuses":[0,200]}}), 'GET')

workbox.routing.registerRoute(new RegExp('https://fonts.googleapis.com/.*'), workbox.strategies.cacheFirst({"cacheableResponse":{"statuses":[0,200]}}), 'GET')
