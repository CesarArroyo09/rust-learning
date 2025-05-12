# Create a new Rust project and add it to VS Code settings and workspace
new-project:
	@if [ -z "$(folder_name)" ]; then \
		echo "Usage: make new-project folder_name=<dd_name>"; \
		echo "Example: make new-project folder_name=02_functions"; \
		exit 1; \
	fi
	
	@# Extract project name by removing digits and underscore prefix
	@project_name=$$(echo $(folder_name) | sed 's/^[0-9]*_//'); \
	echo "Creating new Rust project: $$project_name at $(folder_name)"; \
	cargo new --name $$project_name $(folder_name)
	
	@# Add project to workspace Cargo.toml if it exists
	@if [ -f Cargo.toml ]; then \
		if ! grep -q "members" Cargo.toml; then \
			echo -e "\n[workspace]\nmembers = [\"$(folder_name)\"]" >> Cargo.toml; \
		elif ! grep -q "\"$(folder_name)\"" Cargo.toml; then \
			sed -i -e '/members = \[/a \ \ \"$(folder_name)\",' Cargo.toml; \
		fi; \
	else \
		echo -e "[workspace]\nmembers = [\"$(folder_name)\"]" > Cargo.toml; \
	fi
	
	@# Update VS Code settings
	@mkdir -p .vscode
	@if [ ! -f .vscode/settings.json ]; then \
		echo '{"rust-analyzer.linkedProjects": []}' > .vscode/settings.json; \
	fi
	
	@# Add the project to rust-analyzer.linkedProjects using plain bash
	@if ! grep -q "\"$(folder_name)/Cargo.toml\"" .vscode/settings.json; then \
		sed -i -e '/\"rust-analyzer.linkedProjects\": \[/a \ \ \ \ \ \ \ \ "$(folder_name)/Cargo.toml",' .vscode/settings.json; \
	fi
	
	@echo "Project $(folder_name) created and configured successfully"

# Create a new Rust library project and add it to VS Code settings and workspace
new-project-lib:
	@if [ -z "$(folder_name)" ]; then \
		echo "Usage: make new-project-lib folder_name=<dd_name>"; \
		echo "Example: make new-project-lib folder_name=02_functions_lib"; \
		exit 1; \
	fi

	@# Extract project name by removing digits and underscore prefix
	@project_name=$$(echo $(folder_name) | sed 's/^[0-9]*_//'); \
	echo "Creating new Rust library project: $$project_name at $(folder_name)"; \
	cargo new --lib --name $$project_name $(folder_name)
    
	@# Add project to workspace Cargo.toml if it exists
	@if [ -f Cargo.toml ]; then \
		if ! grep -q "members" Cargo.toml; then \
			echo -e "\n[workspace]\nmembers = [\"$(folder_name)\"]" >> Cargo.toml; \
		elif ! grep -q "\"$(folder_name)\"" Cargo.toml; then \
			sed -i -e '/members = \[/a \ \ \"$(folder_name)\",' Cargo.toml; \
		fi; \
	else \
		echo -e "[workspace]\nmembers = [\"$(folder_name)\"]" > Cargo.toml; \
	fi

	@# Update VS Code settings
	@mkdir -p .vscode
	@if [ ! -f .vscode/settings.json ]; then \
		echo '{"rust-analyzer.linkedProjects": []}' > .vscode/settings.json; \
	fi

	@# Add the project to rust-analyzer.linkedProjects using plain bash
	@if ! grep -q "\"$(folder_name)/Cargo.toml\"" .vscode/settings.json; then \
		sed -i -e '/\"rust-analyzer.linkedProjects\": \[/a \ \ \ \ \ \ \ \ "$(folder_name)/Cargo.toml",' .vscode/settings.json; \
	fi

	@echo "Library project $(folder_name) created and configured successfully"