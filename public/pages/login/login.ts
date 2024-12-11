const form = document.getElementById("form") as HTMLFormElement;

form.addEventListener("submit", e => {
    e.preventDefault();

    const formData = new FormData(form);

    const name = formData.get("name") as string;
    const email = formData.get("email") as string;
    const password = formData.get("password") as string;

    if (name && email && password) {
        alert(
            "Login successful, your data:" +
                "{ name: " +
                name +
                ", email: " +
                email +
                ", password: " +
                password +
                " }"
        );
    } else {
        alert("Please fill in all fields");
    }
});
