// @ts-check

/** @typedef {{
      name: string,
      path: string,
      regExp: RegExp,
      component?: HTMLElement
    }} Route
 */

/* global self */
/* global HTMLElement */
/* global location */
/* global customElements */

/**
 * https://github.com/Weedshaker/event-driven-web-components-realworld-example-app/blob/master/FRONTEND_INSTRUCTIONS.md#routing-guidelines
 * As a controller, this component becomes a router
 *
 * @export
 * @class Router
 */
export default class Router extends HTMLElement {
  constructor () {
    super()

    /** @type {Route[]} */
    this.routes = [
      // Home page (URL: /#/ )
      {
        name: 'p-home',
        path: '../pages/Home.js',
        regExp: new RegExp(/^#\/$/)
      },
      // Sign in/Sign up pages (URL: /#/login, /#/register )
      {
        name: 'p-login',
        path: '../pages/Login.js',
        regExp: new RegExp(/^#\/login/)
      },
      {
        name: 'p-register',
        path: '../pages/Register.js',
        regExp: new RegExp(/^#\/register/)
      },
      // Settings page (URL: /#/settings )
      {
        name: 'p-settings',
        path: '../pages/Settings.js',
        regExp: new RegExp(/^#\/settings/)
      },
      // Editor page to create/edit articles (URL: /#/editor, /#/editor/article-slug-here )
      {
        name: 'p-editor',
        path: '../pages/Editor.js',
        regExp: new RegExp(/^#\/editor/)
      },
      // Article page (URL: /#/article/article-slug-here )
      {
        name: 'p-article',
        path: '../pages/Article.js',
        regExp: new RegExp(/^#\/article/)
      },
      // Profile page (URL: /#/profile/:username, /#/profile/:username/favorites )
      {
        name: 'p-profile',
        path: '../pages/Profile.js',
        regExp: new RegExp(/^#\/profile/)
      }
    ]

    /**
     * Listens to hash changes and forwards the new hash to route
     */
    this.hashChangeListener = event => this.route(location.hash, false, event.newURL === event.oldURL)
  }

  connectedCallback () {
    self.addEventListener('hashchange', this.hashChangeListener)
    this.route(this.routes.some(route => route.regExp.test(location.hash)) ? location.hash : '#/', true)
  }

  disconnectedCallback () {
    self.removeEventListener('hashchange', this.hashChangeListener)
  }

  /**
   * route to the desired hash/domain
   *
   * @param {string} hash
   * @param {boolean} [replace = false]
   * @param {boolean} [isUrlEqual = true]
   * @return {void | string}
   */
  route (hash, replace = false, isUrlEqual = true) {
    // escape on route call which is not set by hashchange event and trigger it here, if needed
    if (location.hash !== hash) {
      if (replace) return location.replace(hash)
      return (location.hash = hash)
    }
    let route
    // find the correct route or do nothing
    if ((route = this.routes.find(route => route.regExp.test(hash)))) {
      // reuse route.component, if already set, otherwise import and define custom element
      // @ts-ignore
      (route.component ? Promise.resolve(route.component) : import(route.path).then(module => {
        // don't define already existing customElements
        if (!customElements.get(route.name)) customElements.define(route.name, module.default)
        // save it to route object for reuse. grab child if it already exists.
        return (route.component = this.children && this.children[0] && this.children[0].tagName === route.name.toUpperCase() ? this.children[0] : document.createElement(route.name))
      })).then(component => {
        if (this.shouldComponentRender(route.name, isUrlEqual)) this.render(component)
      // @ts-ignore
      }).catch(error => console.warn('Router did not find:', route) || error)
    }
  }

  /**
   * evaluates if a render is necessary
   *
   * @param {string} name
   * @param {boolean} [isUrlEqual = true]
   * @return {boolean}
   */
  shouldComponentRender (name, isUrlEqual = true) {
    if (!this.children || !this.children.length) return true
    return !isUrlEqual || this.children[0].tagName !== name.toUpperCase()
  }

  /**
   * renders the page
   *
   * @param {HTMLElement} component
   * @return {void}
   */
  render (component) {
    // clear previous content
    this.innerHTML = ''
    this.appendChild(component)
  }
}
