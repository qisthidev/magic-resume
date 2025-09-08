class MagicResumeApp {
    constructor() {
        this.currentResumeId = null;
        this.currentResume = null;
        this.init();
    }

    init() {
        this.bindEvents();
        this.showHeroSection();
    }

    bindEvents() {
        // Navigation events
        document.getElementById('create-resume-btn').addEventListener('click', () => this.createNewResume());
        document.getElementById('view-resumes-btn').addEventListener('click', () => this.showResumeList());
        
        // Editor events
        document.getElementById('save-resume-btn').addEventListener('click', () => this.saveResume());
        document.getElementById('export-pdf-btn').addEventListener('click', () => this.exportPDF());
        
        // AI events
        document.getElementById('grammar-check-btn').addEventListener('click', () => this.checkGrammar());
        document.getElementById('polish-text-btn').addEventListener('click', () => this.polishText());
        
        // Form events
        document.getElementById('basic-info-form').addEventListener('input', () => this.updatePreview());
        document.getElementById('skills-content').addEventListener('input', () => this.updatePreview());
        
        // Add section events
        document.getElementById('add-experience-btn').addEventListener('click', () => this.addExperience());
        document.getElementById('add-education-btn').addEventListener('click', () => this.addEducation());
    }

    showHeroSection() {
        document.getElementById('resume-list').style.display = 'none';
        document.getElementById('resume-editor').style.display = 'none';
        document.querySelector('.hero').style.display = 'block';
    }

    async showResumeList() {
        document.querySelector('.hero').style.display = 'none';
        document.getElementById('resume-editor').style.display = 'none';
        document.getElementById('resume-list').style.display = 'block';
        
        await this.loadResumes();
    }

    async loadResumes() {
        try {
            const response = await fetch('/api/resumes');
            const resumes = await response.json();
            
            const container = document.getElementById('resumes-container');
            container.innerHTML = '';
            
            if (resumes.length === 0) {
                container.innerHTML = '<p>No resumes found. Create your first resume!</p>';
                return;
            }
            
            resumes.forEach(resume => {
                const card = this.createResumeCard(resume);
                container.appendChild(card);
            });
        } catch (error) {
            console.error('Error loading resumes:', error);
            this.showError('Failed to load resumes');
        }
    }

    createResumeCard(resume) {
        const card = document.createElement('div');
        card.className = 'resume-card';
        card.innerHTML = `
            <h4>${resume.title}</h4>
            <p>Last updated: ${new Date(resume.updated_at).toLocaleDateString()}</p>
            <div class="resume-card-actions">
                <button class="btn btn-sm btn-primary" onclick="app.editResume('${resume.id}')">Edit</button>
                <button class="btn btn-sm btn-secondary" onclick="app.exportResumePDF('${resume.id}')">PDF</button>
                <button class="btn btn-sm" onclick="app.deleteResume('${resume.id}')" style="background: #ef4444; color: white;">Delete</button>
            </div>
        `;
        return card;
    }

    async createNewResume() {
        try {
            const title = prompt('Enter resume title:') || 'Untitled Resume';
            
            const response = await fetch('/api/resumes', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ title }),
            });
            
            if (!response.ok) throw new Error('Failed to create resume');
            
            const resume = await response.json();
            this.editResume(resume.id);
        } catch (error) {
            console.error('Error creating resume:', error);
            this.showError('Failed to create resume');
        }
    }

    async editResume(resumeId) {
        try {
            const response = await fetch(`/api/resumes/${resumeId}`);
            if (!response.ok) throw new Error('Resume not found');
            
            this.currentResume = await response.json();
            this.currentResumeId = resumeId;
            
            this.showEditor();
            this.populateEditor();
        } catch (error) {
            console.error('Error loading resume:', error);
            this.showError('Failed to load resume');
        }
    }

    showEditor() {
        document.querySelector('.hero').style.display = 'none';
        document.getElementById('resume-list').style.display = 'none';
        document.getElementById('resume-editor').style.display = 'block';
    }

    populateEditor() {
        if (!this.currentResume) return;
        
        const basic = this.currentResume.basic;
        
        // Populate basic info
        document.getElementById('name').value = basic.name || '';
        document.getElementById('title').value = basic.title || '';
        document.getElementById('email').value = basic.email || '';
        document.getElementById('phone').value = basic.phone || '';
        document.getElementById('location').value = basic.location || '';
        
        // Populate skills
        document.getElementById('skills-content').value = this.currentResume.skill_content || '';
        
        // Populate experience
        this.populateExperience();
        
        // Populate education
        this.populateEducation();
        
        // Update preview
        this.updatePreview();
    }

    populateExperience() {
        const experienceList = document.getElementById('experience-list');
        experienceList.innerHTML = '';
        
        const experiences = this.currentResume.experience || [];
        experiences.forEach((exp, index) => {
            const expDiv = document.createElement('div');
            expDiv.className = 'form-group';
            expDiv.innerHTML = `
                <div class="experience-item" data-index="${index}">
                    <input type="text" placeholder="Company" value="${exp.company || ''}" onchange="app.updateExperience(${index}, 'company', this.value)">
                    <input type="text" placeholder="Position" value="${exp.position || ''}" onchange="app.updateExperience(${index}, 'position', this.value)">
                    <input type="text" placeholder="Date" value="${exp.date || ''}" onchange="app.updateExperience(${index}, 'date', this.value)">
                    <textarea placeholder="Details" onchange="app.updateExperience(${index}, 'details', this.value)">${exp.details || ''}</textarea>
                    <button type="button" class="btn btn-sm" onclick="app.removeExperience(${index})" style="background: #ef4444; color: white;">Remove</button>
                </div>
            `;
            experienceList.appendChild(expDiv);
        });
    }

    populateEducation() {
        const educationList = document.getElementById('education-list');
        educationList.innerHTML = '';
        
        const educations = this.currentResume.education || [];
        educations.forEach((edu, index) => {
            const eduDiv = document.createElement('div');
            eduDiv.className = 'form-group';
            eduDiv.innerHTML = `
                <div class="education-item" data-index="${index}">
                    <input type="text" placeholder="School" value="${edu.school || ''}" onchange="app.updateEducation(${index}, 'school', this.value)">
                    <input type="text" placeholder="Major" value="${edu.major || ''}" onchange="app.updateEducation(${index}, 'major', this.value)">
                    <input type="text" placeholder="Degree" value="${edu.degree || ''}" onchange="app.updateEducation(${index}, 'degree', this.value)">
                    <input type="text" placeholder="Start Date" value="${edu.start_date || ''}" onchange="app.updateEducation(${index}, 'start_date', this.value)">
                    <input type="text" placeholder="End Date" value="${edu.end_date || ''}" onchange="app.updateEducation(${index}, 'end_date', this.value)">
                    <button type="button" class="btn btn-sm" onclick="app.removeEducation(${index})" style="background: #ef4444; color: white;">Remove</button>
                </div>
            `;
            educationList.appendChild(eduDiv);
        });
    }

    addExperience() {
        if (!this.currentResume.experience) {
            this.currentResume.experience = [];
        }
        
        this.currentResume.experience.push({
            id: Date.now().toString(),
            company: '',
            position: '',
            date: '',
            details: '',
            visible: true
        });
        
        this.populateExperience();
    }

    addEducation() {
        if (!this.currentResume.education) {
            this.currentResume.education = [];
        }
        
        this.currentResume.education.push({
            id: Date.now().toString(),
            school: '',
            major: '',
            degree: '',
            start_date: '',
            end_date: '',
            visible: true
        });
        
        this.populateEducation();
    }

    updateExperience(index, field, value) {
        if (this.currentResume.experience && this.currentResume.experience[index]) {
            this.currentResume.experience[index][field] = value;
            this.updatePreview();
        }
    }

    updateEducation(index, field, value) {
        if (this.currentResume.education && this.currentResume.education[index]) {
            this.currentResume.education[index][field] = value;
            this.updatePreview();
        }
    }

    removeExperience(index) {
        if (this.currentResume.experience) {
            this.currentResume.experience.splice(index, 1);
            this.populateExperience();
            this.updatePreview();
        }
    }

    removeEducation(index) {
        if (this.currentResume.education) {
            this.currentResume.education.splice(index, 1);
            this.populateEducation();
            this.updatePreview();
        }
    }

    updatePreview() {
        if (!this.currentResume) return;
        
        // Update basic info from form
        const basic = this.currentResume.basic;
        basic.name = document.getElementById('name').value;
        basic.title = document.getElementById('title').value;
        basic.email = document.getElementById('email').value;
        basic.phone = document.getElementById('phone').value;
        basic.location = document.getElementById('location').value;
        
        // Update skills
        this.currentResume.skill_content = document.getElementById('skills-content').value;
        
        // Generate preview HTML
        const previewHtml = this.generatePreviewHtml();
        document.getElementById('resume-preview').innerHTML = previewHtml;
    }

    generatePreviewHtml() {
        const basic = this.currentResume.basic;
        const experiences = this.currentResume.experience || [];
        const educations = this.currentResume.education || [];
        const skills = this.currentResume.skill_content || '';
        
        return `
            <div class="resume-preview">
                <div class="resume-header">
                    <h1>${basic.name || 'Your Name'}</h1>
                    <div class="title">${basic.title || 'Professional Title'}</div>
                    <div class="resume-contact">
                        ${basic.email ? `<span>📧 ${basic.email}</span>` : ''}
                        ${basic.phone ? `<span>📞 ${basic.phone}</span>` : ''}
                        ${basic.location ? `<span>📍 ${basic.location}</span>` : ''}
                    </div>
                </div>
                
                ${experiences.length > 0 ? `
                    <div class="resume-section">
                        <h2>Experience</h2>
                        ${experiences.map(exp => `
                            <div class="resume-item">
                                <h3>${exp.company}</h3>
                                <div class="position">${exp.position}</div>
                                <div class="date">${exp.date}</div>
                                <div class="description">${exp.details}</div>
                            </div>
                        `).join('')}
                    </div>
                ` : ''}
                
                ${educations.length > 0 ? `
                    <div class="resume-section">
                        <h2>Education</h2>
                        ${educations.map(edu => `
                            <div class="resume-item">
                                <h3>${edu.school}</h3>
                                <div class="position">${edu.major} - ${edu.degree}</div>
                                <div class="date">${edu.start_date} - ${edu.end_date}</div>
                            </div>
                        `).join('')}
                    </div>
                ` : ''}
                
                ${skills ? `
                    <div class="resume-section">
                        <h2>Skills</h2>
                        <div class="description">${skills}</div>
                    </div>
                ` : ''}
            </div>
        `;
    }

    async saveResume() {
        if (!this.currentResumeId || !this.currentResume) return;
        
        try {
            const response = await fetch(`/api/resumes/${this.currentResumeId}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    basic: this.currentResume.basic,
                    experience: this.currentResume.experience,
                    education: this.currentResume.education,
                    skill_content: this.currentResume.skill_content,
                }),
            });
            
            if (!response.ok) throw new Error('Failed to save resume');
            
            this.showSuccess('Resume saved successfully!');
        } catch (error) {
            console.error('Error saving resume:', error);
            this.showError('Failed to save resume');
        }
    }

    async exportPDF() {
        if (!this.currentResumeId) return;
        
        try {
            const response = await fetch(`/api/resumes/${this.currentResumeId}/export/pdf`);
            if (!response.ok) throw new Error('Failed to export PDF');
            
            const blob = await response.blob();
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `${this.currentResume.title || 'resume'}.pdf`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Error exporting PDF:', error);
            this.showError('Failed to export PDF');
        }
    }

    async exportResumePDF(resumeId) {
        try {
            const response = await fetch(`/api/resumes/${resumeId}/export/pdf`);
            if (!response.ok) throw new Error('Failed to export PDF');
            
            const blob = await response.blob();
            const url = window.URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.href = url;
            a.download = `resume-${resumeId}.pdf`;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            window.URL.revokeObjectURL(url);
        } catch (error) {
            console.error('Error exporting PDF:', error);
            this.showError('Failed to export PDF');
        }
    }

    async deleteResume(resumeId) {
        if (!confirm('Are you sure you want to delete this resume?')) return;
        
        try {
            const response = await fetch(`/api/resumes/${resumeId}`, {
                method: 'DELETE',
            });
            
            if (!response.ok) throw new Error('Failed to delete resume');
            
            this.showSuccess('Resume deleted successfully!');
            this.loadResumes();
        } catch (error) {
            console.error('Error deleting resume:', error);
            this.showError('Failed to delete resume');
        }
    }

    async checkGrammar() {
        const skillsContent = document.getElementById('skills-content').value;
        if (!skillsContent.trim()) {
            this.showError('Please enter some text to check grammar');
            return;
        }
        
        try {
            const response = await fetch('/api/grammar', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ text: skillsContent }),
            });
            
            if (!response.ok) throw new Error('Grammar check failed');
            
            const result = await response.json();
            document.getElementById('skills-content').value = result.corrected_text;
            this.updatePreview();
            this.showSuccess('Grammar checked and corrected!');
        } catch (error) {
            console.error('Error checking grammar:', error);
            this.showError('Failed to check grammar');
        }
    }

    async polishText() {
        const skillsContent = document.getElementById('skills-content').value;
        if (!skillsContent.trim()) {
            this.showError('Please enter some text to polish');
            return;
        }
        
        try {
            const response = await fetch('/api/polish', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ 
                    text: skillsContent,
                    style: 'professional'
                }),
            });
            
            if (!response.ok) throw new Error('Text polishing failed');
            
            const result = await response.json();
            document.getElementById('skills-content').value = result.polished_text;
            this.updatePreview();
            this.showSuccess('Text polished successfully!');
        } catch (error) {
            console.error('Error polishing text:', error);
            this.showError('Failed to polish text');
        }
    }

    showError(message) {
        this.showMessage(message, 'error');
    }

    showSuccess(message) {
        this.showMessage(message, 'success');
    }

    showMessage(message, type) {
        // Remove existing messages
        const existing = document.querySelectorAll('.message');
        existing.forEach(el => el.remove());
        
        const messageEl = document.createElement('div');
        messageEl.className = `message ${type}`;
        messageEl.textContent = message;
        
        document.body.insertBefore(messageEl, document.body.firstChild);
        
        setTimeout(() => {
            messageEl.remove();
        }, 5000);
    }
}

// Initialize the app
const app = new MagicResumeApp();