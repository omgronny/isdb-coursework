var JediRequestApi = Vue.resource('/request{/id}');

Vue.component('jedi-requests-list', {
    props: ['requests'],
    template:
        '<div>' +
        '<table>' +
        '<tr><th colspan="4" width="70%">Jedi Requests</th><th></th></tr>' +
        '<tr class="timer"><td> Jedi </td><td> Price </td></tr>' +
        '<tr v-for="request in requests">' +
        '<td><i>{{ request[0] }}</i></td><td> {{ request[1] }} </td>' +
        '</tr>' +
        '</table>' +
        '</div>',
    created: function () {
        JediRequestApi.get().then(result =>
            result.json().then(data =>
                data.forEach(msg => this.requests.push(msg))
            )
        )
    }
});