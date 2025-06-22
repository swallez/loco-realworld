// @ts-check

/* global customElements */
/* global HTMLElement */

/**
 * https://github.com/Weedshaker/event-driven-web-components-realworld-example-app/blob/master/FRONTEND_INSTRUCTIONS.md#ListArticles
 * As an organism, this component shall hold molecules and/or atoms
 * this organism always renders new when connected to keep most recent and does not need shouldComponentRender
 *
 * @export
 * @class ListArticlePreviews
 */
export default class ListArticlePreviews extends HTMLElement {
  constructor () {
    super()

    /**
     * Listens to the event name/typeArg: 'listArticles'
     *
     * @param {CustomEvent & {detail: import("../controllers/Article.js").ListArticlesEventDetail}} event
     */
    this.listArticlesListener = event => this.render(event.detail.fetch)
  }

  connectedCallback () {
    // listen for articles
    document.body.addEventListener('listArticles', this.listArticlesListener)
    // is not needed since the molecules/Pagination.js, which is located after this, does the same request
    // it is possible to have multiple components request the same data on connectCallback but then it ether should expect a private response by a promise with caching or tolerate abort
    // on every connect it will attempt to get newest articles
    // this.dispatchEvent(new CustomEvent('requestListArticles', {
    //   /** @type {import("../controllers/Article.js").RequestListArticlesEventDetail} */
    //   detail: {},
    //   bubbles: true,
    //   cancelable: true,
    //   composed: true
    // }))
  }

  disconnectedCallback () {
    document.body.removeEventListener('listArticles', this.listArticlesListener)
  }

  /**
   * renders each received article
   *
   * @param {Promise<import("../../helpers/Interfaces.js").MultipleArticles>} fetchMultipleArticles
   * @return {void}
   */
  render (fetchMultipleArticles) {
    Promise.all([fetchMultipleArticles, this.loadChildComponents()]).then(result => {
      const [multipleArticles, children] = result
      if (!multipleArticles || !multipleArticles.articles || !multipleArticles.articles.length) {
        this.innerHTML = '<div class="article-preview">No articles are here... yet.</div>'
      } else {
        this.innerHTML = ''
        multipleArticles.articles.forEach(article => {
          /** @type {import("../molecules/ArticlePreview.js").default & any} */
          // @ts-ignore
          const articlePreview = new children[0][1](article)
          this.appendChild(articlePreview)
        })
        if (!this.getAttribute('no-scroll')) this.scrollToEl(this)
      }
    // @ts-ignore
    }).catch(error => (this.innerHTML = console.warn(error) || (error && typeof error.toString === 'function' && error.toString().includes('aborted') ? '<div class="article-preview">Loading...</div>' : '<div class="article-preview">An error occurred fetching the articles!</div>')))
  }

  /**
   * fetch children when first needed
   *
   * @returns {Promise<[string, CustomElementConstructor][]>}
   */
  loadChildComponents () {
    return this.childComponentsPromise || (this.childComponentsPromise = Promise.all([
      import('../molecules/ArticlePreview.js').then(
        /** @returns {[string, CustomElementConstructor]} */
        module => ['m-article-preview', module.default]
      )
    ]).then(elements => {
      elements.forEach(element => {
        // don't define already existing customElements
        // @ts-ignore
        if (!customElements.get(element[0])) customElements.define(...element)
      })
      return elements
    }))
  }

  // inspired from: https://github.com/Weedshaker/PeerWebSite/blob/master/JavaScript/js/Player/Player.js
  scrollToEl (el) {
    const rect = el.getBoundingClientRect()
    // check if the element is outside the viewport, otherwise don't scroll
    if (rect && rect.top < 0) el.scrollIntoView({ block: 'start', inline: 'nearest', behavior: 'smooth' })
  }
}
