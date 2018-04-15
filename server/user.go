package main

type User struct {
	ID    string
	Email string
}

func InsertUser(user *User) error {
	var id string
	err := db.QueryRow(`
    INSERT INTO users(email)
    VALUES ($1)
    RETURNING id
  `, user.Email).Scan(&id)
	if err != nil {
		return err
	}
	user.ID = id
	return nil
}

func GetUserByID(id string) (*User, error) {
	var email string
	err := db.QueryRow("SELECT email FROM users WHERE id=$1", id).Scan(&email)
	if err != nil {
		return nil, err
	}
	return &User{
		ID:    id,
		Email: email,
	}, nil
}

func RemoveUserByID(id string) error {
	_, err := db.Exec("DELETE FROM users WHERE id=$1", id)
	return err
}
