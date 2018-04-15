package api

type TestConnection struct {
	url string
}

func (conn *TestConnection) Init() {
}

func (conn *TestConnection) GetUserByID(id string) (*User, error) {
	return &User{
		ID:    id,
		Email: "some@email.test",
	}, nil
}

func (conn *TestConnection) GetViewer() (*User, error) {
	return &User{
		ID:    "1234",
		Name:  "Gnusto",
		Email: "gnusto@tyrell.test",
	}, nil
}

func (conn *TestConnection) InsertUser(user *User) error {
	return nil
}

func (conn *TestConnection) RemoveUserByID(id string) error {
	return nil
}
