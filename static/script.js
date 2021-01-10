const app = new Vue({
  el: '#app',
  data: {
    status: {},
    orders: [],
    newOrder: {
      name: null,
      target: null,
      command: null
    }
  },
  methods: {
    refreshStatus: function () {
      fetch('/status')
        .then(res => res.json())
        .then(data => {
          this.status = data
        })
        .catch(e => alert(e))
    },
    refreshList: function () {
      fetch('/orders')
        .then(res => res.json())
        .then(data => {
          this.orders = data
        })
        .catch(e => alert(e))
    },
    registerNewCommand: function () {
      fetch('/orders', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(this.newOrder)
      })
        .then(() => this.refreshList())
        .catch(e => alert(e))
    },
    removeCommand: function (id) {
      if (!id) {
        alert('Invalid ID')
        return
      }

      fetch(`/orders/${id}`, {
        method: 'DELETE'
      })
        .then(() => this.refreshList())
        .catch(e => alert(e))
    }
  },
  mounted: function () {
    this.refreshStatus()
    this.refreshList()
  }
})
