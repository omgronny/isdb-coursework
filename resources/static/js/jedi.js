var jediApi = Vue.resource('/jedi{/id}');

Vue.component('jedi-list', {
    props: ['jedi'],
    template:
        '<div>' +
        '<table>' +
        '<tr><th colspan="4" width="70%">Jedi</th><th></th></tr>' +
        '<tr class="timer"><td> ID </td><td> Name </td><td> Power </td></tr>' +
        '<tr v-for="jd in jedi">' +
        '<td><i>{{ jd.id }}</i></td><td> {{ jd.name }} </td><td>{{ jd.power }}</td>' +
        '</tr>' +
        '</table>' +
        '</div>',
    created: function () {
        jediApi.get().then(result =>
            result.json().then(data =>
                data.forEach(msg => this.jedi.push(msg))
            )
        )
    }
});