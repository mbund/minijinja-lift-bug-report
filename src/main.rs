use minijinja::{context, Environment, State};

fn main() {
    let mut env = Environment::new();
    env.add_template(
        "main",
        r#"
        {% macro wrapper() %} <wrapper>{{ caller() }}</wrapper> {% endmacro %}

        {{ myfunc() }}

        {% call wrapper() %}
            {{ var_a }}
            {{ myfunc() }}
        {% endcall %}

        {% call wrapper() %}
            {% with lift_global_up=my_global %}{% endwith %}
            {{ var_a }}
            {{ myfunc() }}
        {% endcall %}
        "#,
    )
    .unwrap();
    env.add_function("myfunc", jinja_myfunc);
    let template = env.get_template("main").unwrap();
    println!(
        "{}",
        template
            .render(context! { var_a => "v_a", my_global => "my_global" })
            .unwrap()
    );
}

fn jinja_myfunc(state: &State) -> String {
    let global = state.lookup("my_global");

    match global {
        Some(_) => "global exists".to_owned(),
        None => "global DOES NOT exist".to_owned(),
    }
}
