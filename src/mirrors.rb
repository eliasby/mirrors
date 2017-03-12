require 'json'

class Program
    def self.run
        file = File.read('mirrors.json')
        json = JSON.parse(file)
        
        primary = json['primary']
        mirrors = json['mirrors']

        system 'git remote rm origin'
        system "git remote add origin #{primary}"
        system "git remote set-url --add --push origin #{primary}"

        mirrors.each do |mirror|
            system "git remote set-url --add --push origin #{mirror}"
        end

        system 'git remote -v'
    end
end