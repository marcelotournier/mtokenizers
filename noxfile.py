import nox


@nox.session(venv_backend="conda")
def tests(session):
    session.install('pytest', "maturin")
    session.run("cargo", "test")
    session.run("maturin", "develop")
    session.run('pytest')


@nox.session(venv_backend="conda")
def lint(session):
    session.install('flake8')
    session.run('flake8')
