
var amountApi = Vue.resource('/hunter-id{/id}');
Vue.component('hunter-money', {
    data: function() {
        return {
            id: '',
            in_money: ''
        }
    },
    template:
        '<div>' +
        '<table>' +
        '<tr><th>My Money</th></tr>' +
        '<tr><td>{{ in_money }}</td></tr>' +
        '</table>' +
        '</div>',
    created: function () {
        this.id = 1;
        amountApi.get({ id: this.id }).then(result =>
            result.json().then(data =>
                data.forEach(amount => this.in_money = amount)
            )
        )
    }
});


var app = new Vue({
    el: '#app',
    template:
        '<div>' +
            '<div class=".overlay">' +
                '<hunter-money />' +
                '<jedi-requests-list :requests="requests" />' +
                '<jedi-list :jedi="jedi" />' +
            '</div>' +

            '<div class="sol">\n' +
        '\t\t<div class="tie tie-1">\n' +
        '\t\t\t<div class="tie-asa asa-esq"></div>\n' +
        '\t\t\t<div class="tie-asa-meio">\n' +
        '\t\t\t\t<div class="tie-cabine"></div>\n' +
        '\t\t\t</div>\n' +
        '\t\t\t<div class="tie-asa asa-dir"></div>\n' +
        '\t\t</div>\n' +
        '\t\t<div class="tie tie-2">\n' +
        '\t\t\t<div class="tie-asa asa-esq"></div>\n' +
        '\t\t\t<div class="tie-asa-meio">\n' +
        '\t\t\t\t<div class="tie-cabine"></div>\n' +
        '\t\t\t</div>\n' +
        '\t\t\t<div class="tie-asa asa-dir"></div>\n' +
        '\t\t</div>\n' +
        '\t\t<div class="tie tie-3">\n' +
        '\t\t\t<div class="tie-asa asa-esq"></div>\n' +
        '\t\t\t<div class="tie-asa-meio">\n' +
        '\t\t\t\t<div class="tie-cabine"></div>\n' +
        '\t\t\t</div>\n' +
        '\t\t\t<div class="tie-asa asa-dir"></div>\n' +
        '\t\t</div>\n' +
        '\t\t<div class="tie-diferente"></div>\n' +
        '\t\t<div class="tie-diferente-b"></div>\n' +
        '\t</div>\n' +
        // '\t\t<div class="montanha-1"></div>\n' +
        // '\t\t<div class="montanha-2"></div>\n' +
        // '\t\t<div class="montanha-3"></div>\n' +
        // '\t\t<div class="montanha-4"></div>\n' +
        // '\t\t<div class="base-montanha-1"></div>\n' +
        '\t\t<div class="montanha-1b"></div>\n' +
        '\t\t<div class="montanha-2b"></div>\n' +
        '\t\t<div class="montanha-3b"></div>\n' +
        '\t\t<div class="montanha-4b"></div>' +

        '</div>'
    ,
    data: {
        messages: [],
        requests: [],
        jedi: [],
        my_jedi_data: [],
        jedi_pre_data: []
    }
});




